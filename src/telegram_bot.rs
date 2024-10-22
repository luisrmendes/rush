use crate::{
    commands::{self},
    llm_wrapper::Llm,
    GlobalState, Systems,
};
use log::debug;
use std::{error::Error, str::FromStr};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use strum::{EnumIter, IntoEnumIterator};
use teloxide::{
    prelude::{Request, Requester},
    types::ChatId,
    Bot,
};
use tokio::{sync::Mutex, time::sleep};

#[derive(Debug, EnumIter)]
pub enum Command {
    GetIpv4,
    StatusSnowdog,
    LightsOn,
    LightsOff,
}

//impl Executable for Command {}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::GetIpv4 => String::from("/ipv4"),
            Command::StatusSnowdog => String::from("/status_snowdog"),
            Command::LightsOn => String::from("/lights_on"),
            Command::LightsOff => String::from("/lights_off"),
        }
    }
}

impl FromStr for Command {
    type Err = ();
    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "/ipv4" => Ok(Command::GetIpv4),
            "/status_snowdog" => Ok(Command::StatusSnowdog),
            "/lights_on" => Ok(Command::LightsOn),
            "/lights_off" => Ok(Command::LightsOff),
            _ => Err(()),
        }
    }
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
    /// I'm home if my phone is connected to the local network
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

    async fn execute(ctx: &Systems, op: &Command) -> Result<String, Box<dyn Error>> {
        match op {
            Command::GetIpv4 => Ok(commands::get_ipv4().await?),
            Command::StatusSnowdog => match commands::is_online(&ctx.pcs[1].clone()) {
                Ok(out) => Ok(out.to_string()),
                Err(e) => Err(e.into()),
            },
            Command::LightsOn => Ok(commands::lights_on().await?),
            Command::LightsOff => Ok(commands::lights_off().await?),
        }
    }

    pub async fn run_repl(&self) {
        use teloxide::prelude::*;

        let context_arc = Arc::new(self.context.clone());
        let llm_mutex = Arc::new(Mutex::new(self.llm.clone()));

        teloxide::repl(self.bot.clone(), move |bot: Bot, msg: Message| {
            let context_clone = Arc::clone(&context_arc);
            let llm_clone = Arc::clone(&llm_mutex);

            async move {
                debug!("Received from bot: {:?}", msg.text());

                bot.send_message(msg.chat.id, "...").await?;

                let Some(text) = msg.text() else {
                    bot.send_message(msg.chat.id, "?").await?;
                    return Ok(());
                };

                if let Some(char) = text.chars().next() {
                    if char == '/' {
                        let Ok(command) = Command::from_str(text) else {
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

                    let prompt_result = match llm_clone.lock().await.send_prompt(text).await {
                        Ok(res) => res,
                        Err(e) => {
                            format!("Something bad happened connecting to my llm. Error: {e}")
                        }
                    };

                    // Try to find a corresponding command from the prompt
                    let mut command_list = String::new();
                    for cmd in Command::iter() {
                        let cmd_string = cmd.to_string();
                        command_list += &format!("{cmd_string}\n");
                    }

                    let get_command_from_prompt =
                        "This prompt might have a request that correlates to one of these commands: \n".to_owned()
                        + &command_list
                        + "Answer just with the command you believe fits best from the prompt. Answer \"undefined\" if you cannot find any correlation\n"
                        + "Prompt: "
                        + text;

                    let get_command_from_prompt_result = match llm_clone.lock().await.send_prompt(&get_command_from_prompt).await {
                        Ok(res) => res,
                        Err(e) => {
                            format!("Something bad happened connecting to my llm. Error: {e}")
                        }
                    };

                    if let Ok(cmd) = Command::from_str(&get_command_from_prompt_result) {
                       match Self::execute(&context_clone, &cmd).await {
                                Ok(output) => output,
                                Err(e) => {
                                    debug!("Error executing command: {e}");
                                    format!("Error: {e}")
                                }
                            };
                            bot.send_message(CHAT_ID, format!("Ok, doing {get_command_from_prompt_result}")).await?;
                            let _ = Self::execute(&context_clone,&cmd).await;
                        } else {
                            bot.send_message(CHAT_ID, prompt_result).await?;
                            debug!("Command not infered");
                        }
                    }
                else {
                    bot.send_message(msg.chat.id, "Did you sent an empty message?").await?;
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
