use crate::{
    commands::{self},
    llm_wrapper::Llm,
    GlobalState, Systems,
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
    context: Systems,
    global_state: Arc<Mutex<GlobalState>>,
    llm: Llm,
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

    async fn execute(ctx: &Systems, op: &Operation) -> Result<String, String> {
        match op {
            Operation::SuspendSnowdog => commands::suspend(ctx.pcs[1].clone()).await,
            Operation::WakeupSnowdog => commands::wakeup(ctx.pcs[1].clone()).await,
            Operation::StatusSnowdog => match commands::is_online(&ctx.pcs[1].clone()) {
                Ok(out) => Ok(out.to_string()),
                Err(e) => Err(e),
            },
            Operation::GetIpv4 => commands::get_ipv4().await,
        }
    }

    fn parse_commands(text: &str) -> Result<Operation, String> {
        match text {
            "/ipv4" => Ok(Operation::GetIpv4),
            "/wakeup_snowdog" => Ok(Operation::WakeupSnowdog),
            "/status_snowdog" => Ok(Operation::StatusSnowdog),
            "/suspend_snowdog" => Ok(Operation::SuspendSnowdog),
            other => Err(format!("Unknown command {other}")),
        }
    }

    pub async fn run_repl(&self) {
        use teloxide::prelude::*;

        let context_arc = Arc::new(self.context.clone());
        let llm_arc = Arc::new(self.llm.clone());

        teloxide::repl(self.bot.clone(), move |bot: Bot, msg: Message| {
            let context_clone = Arc::clone(&context_arc);
            let llm_clone = Arc::clone(&llm_arc);

            async move {
                debug!("Received from bot: {:?}", msg.text());

                let Some(text) = msg.text() else {
                    bot.send_message(msg.chat.id, "?").await?;
                    return Ok(());
                };

                if let Some(char) = text.chars().next() {
                    if char == '/' {
                        let Ok(command) = Self::parse_commands(text) else {
                            let reply = "Failed to parse command".to_owned();
                            debug!("{reply}");
                            bot.send_message(msg.chat.id, reply).await?;
                            return Ok(());
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
                        return Ok(());
                    }
                    bot.send_message(CHAT_ID, &llm_clone.send_prompt(text).await)
                        .await?;
                } else {
                    bot.send_message(msg.chat.id, "?").await?;
                    return Ok(());
                }

                Ok(())
            }
        })
        .await;
    }

    pub async fn new(context: Systems, global_state: Arc<Mutex<GlobalState>>, llm: Llm) -> Self {
        let bot = Bot::from_env();
        let _ = bot.send_message(CHAT_ID, "Hey I am up!").send().await;

        Self {
            bot,
            context,
            global_state,
            llm,
        }
    }
}
