use crate::{
    commands::{self},
    Context, GlobalState,
};
use log::debug;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use teloxide::{
    prelude::{Request, Requester},
    types::ChatId,
    Bot,
};
use tokio::{sync::Mutex, time::sleep};

#[derive(Debug)]
pub enum Operation {
    GetIpv4,
    WakeupSnowdog,
    StatusSnowdog,
    SuspendSnowdog,
}
#[derive(Clone)]
pub struct TelegramBot {
    bot: Bot,
    context: Context,
    global_state: Arc<Mutex<GlobalState>>,
}

const CHAT_ID: ChatId = ChatId(322_011_297);

impl TelegramBot {
    /// Loop function that messages the telegram bot when I'm home or not
    /// I'm home if my phone is connected to the local networku
    pub async fn update_am_i_home(&self) {
        loop {
            static STORE_AM_I_ONLINE_STATE: AtomicBool = AtomicBool::new(false);
            let state = self.global_state.lock().await.am_i_home;
            debug!("Am I home? {state}");

            if state != STORE_AM_I_ONLINE_STATE.load(Ordering::Relaxed) {
                if state {
                    let _ = self
                        .bot
                        .send_message(CHAT_ID, "You are at home!")
                        .send()
                        .await;
                } else {
                    let _ = self
                        .bot
                        .send_message(CHAT_ID, "You are not home!")
                        .send()
                        .await;
                }
                STORE_AM_I_ONLINE_STATE.store(state, Ordering::SeqCst);
            }
            sleep(Duration::from_millis(1000)).await;
        }
    }

    async fn execute(ctx: &Context, op: &Operation) -> Result<String, String> {
        match op {
            Operation::SuspendSnowdog => commands::suspend(ctx.systems[1].clone()).await,
            Operation::WakeupSnowdog => commands::wakeup(ctx.systems[1].clone()).await,
            Operation::StatusSnowdog => match commands::is_online(&ctx.systems[1].clone()) {
                Ok(out) => Ok(out.to_string()),
                Err(e) => Err(e),
            },
            Operation::GetIpv4 => commands::get_ipv4().await,
        }
    }

    fn parse_commands(text: Option<&str>) -> Result<Operation, String> {
        let parsed_msg: &str = text.unwrap_or_default();
        match parsed_msg {
            "/ipv4" => Ok(Operation::GetIpv4),
            "/wakeup_snowdog" => Ok(Operation::WakeupSnowdog),
            "/status_snowdog" => Ok(Operation::StatusSnowdog),
            "/suspend_snowdog" => Ok(Operation::SuspendSnowdog),
            other => Err(format!("Unknown command {other}")),
        }
    }

    pub async fn answer_commands(&self) {
        use teloxide::prelude::*;

        let context_arc = Arc::new(self.context.clone());

        teloxide::repl(self.bot.clone(), move |bot: Bot, msg: Message| {
            let context_clone = Arc::clone(&context_arc);
            async move {
                debug!("Received from bot: {:?}", msg.text());

                let command = match Self::parse_commands(msg.text()) {
                    Ok(cmd) => cmd,
                    Err(e) => {
                        let reply = format!("Failed to parse command. Error: {e}");
                        debug!("{reply}");
                        bot.send_message(msg.chat.id, reply).await?;
                        return Ok(());
                    }
                };

                let cmd_output = match Self::execute(&context_clone, &command).await {
                    Ok(output) => output,
                    Err(e) => {
                        debug!("Error executing command: {e}");
                        format!("Error: {e}")
                    }
                };

                debug!("Sending to bot: {:?}", cmd_output);
                bot.send_message(msg.chat.id, cmd_output).await?;
                Ok(())
            }
        })
        .await;
    }

    pub async fn new(context: Context, global_state: Arc<Mutex<GlobalState>>) -> Self {
        let bot = Bot::from_env();
        let _ = bot.send_message(CHAT_ID, "Hey I am up!").send().await;

        Self {
            bot,
            context,
            global_state,
        }
    }
}
