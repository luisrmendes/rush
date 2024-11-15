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
use strum::EnumIter;
use teloxide::{
    net::Download,
    prelude::{Request, Requester},
    types::{ChatId, Message},
    Bot,
};
use tokio::{fs, sync::Mutex, time::sleep};

#[derive(Debug, EnumIter)]
pub enum Command {
    GetIpv4,
    LightsOnAll,
    LightsOffAll,
    LightsOnLivingRoom,
    LightsOffLivingRoom,
    LightsOnHall,
    LightsOffHall,
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::GetIpv4 => String::from("/ipv4"),
            Command::LightsOnAll => String::from("/lights_on_all"),
            Command::LightsOffAll => String::from("/lights_off_all"),
            Command::LightsOnLivingRoom => String::from("/lights_on_living_room"),
            Command::LightsOffLivingRoom => String::from("/lights_off_living_room"),
            Command::LightsOnHall => String::from("/lights_on_hall"),
            Command::LightsOffHall => String::from("/lights_off_hall"),
        }
    }
}

impl FromStr for Command {
    type Err = ();
    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "/ipv4" => Ok(Command::GetIpv4),
            "/lights_on_all" => Ok(Command::LightsOnAll),
            "/lights_off_all" => Ok(Command::LightsOffAll),
            "/lights_on_living_room" => Ok(Command::LightsOnLivingRoom),
            "/lights_off_living_room" => Ok(Command::LightsOffLivingRoom),
            "/lights_on_hall" => Ok(Command::LightsOnHall),
            "/lights_off_hall" => Ok(Command::LightsOffHall),
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
    // fn handle_message_prompts(
    //     msg: &Message,
    //     bot: &teloxide::Bot,
    // ) -> Result<String, Box<dyn Error>> {
    // }

    async fn handle_voice_messages(
        msg: &Message,
        bot: &teloxide::Bot,
    ) -> Result<String, Box<dyn Error>> {
        let Some(voice) = msg.voice() else {
            bot.send_message(msg.chat.id, "?").await?;
            return Ok(String::new());
        };
        let file_id = &voice.file.id;

        let file = bot.get_file(file_id).await?;
        let mut dst = fs::File::create(format!("/tmp/{file_id}.ogg")).await?;
        bot.download_file(&file.path, &mut dst).await?;

        // give this file to whisper
        let _response = commands::send_command(&format!(".venv/bin/whisper /tmp/{file_id}.ogg --output_format txt --output_dir /tmp --model base"), None).await;

        let text: String = fs::read_to_string(format!("/tmp/{file_id}.txt")).await?;
        let text = &text[..text.len() - 1];
        bot.send_message(msg.chat.id, format!("Heard \"{text}\""))
            .await?;
        Ok(text.to_owned())
    }

    /// Loop function that messages the telegram bot when I'm home or not
    /// I'm home if my phone is connected to the local network
    pub async fn update_home_presence(&self) {
        loop {
            static STORE_AM_I_HOME_STATE: AtomicBool = AtomicBool::new(false);
            static STORE_IS_SHE_HOME_STATE: AtomicBool = AtomicBool::new(false);

            let am_i_home = self.global_state.lock().await.am_i_home;
            let is_she_home = self.global_state.lock().await.is_she_home;

            debug!("Am I home? {am_i_home}");

            if am_i_home != STORE_AM_I_HOME_STATE.load(Ordering::Relaxed) {
                if am_i_home {
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
                STORE_AM_I_HOME_STATE.store(am_i_home, Ordering::SeqCst);
            }

            if is_she_home != STORE_IS_SHE_HOME_STATE.load(Ordering::Relaxed) {
                if is_she_home {
                    let _ = self.bot.send_message(CHAT_ID, "She's home!").send().await;
                } else {
                    let _ = self
                        .bot
                        .send_message(CHAT_ID, "She's not at home!")
                        .send()
                        .await;
                }
                STORE_IS_SHE_HOME_STATE.store(is_she_home, Ordering::SeqCst);
            }
            sleep(Duration::from_secs(1)).await;
        }
    }

    async fn execute(_ctx: &Systems, op: &Command) -> Result<String, Box<dyn Error>> {
        match op {
            Command::GetIpv4 => Ok(commands::get_ipv4().await?),
            Command::LightsOnAll => Ok(commands::ctrl_all_lights(commands::LightCmd::On).await?),
            Command::LightsOffAll => Ok(commands::ctrl_all_lights(commands::LightCmd::Off).await?),
            Command::LightsOnLivingRoom => {
                Ok(commands::ctrl_living_room_lights(commands::LightCmd::On).await?)
            }
            Command::LightsOffLivingRoom => {
                Ok(commands::ctrl_living_room_lights(commands::LightCmd::Off).await?)
            }
            Command::LightsOnHall => Ok(commands::ctrl_hall_lights(commands::LightCmd::On).await?),
            Command::LightsOffHall => {
                Ok(commands::ctrl_hall_lights(commands::LightCmd::Off).await?)
            }
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
                let my_prompt: String;
                if msg.text().is_some() {
                    let Some(text) = msg.text() else {
                        bot.send_message(msg.chat.id, "DEBUG: No text but text?")
                            .await?;
                        return Ok(());
                    };

                    // Handle if bot receives a text command
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
                    }
                    my_prompt = text.to_owned();
                } else if msg.voice().is_some() {
                    my_prompt = (Self::handle_voice_messages(&msg, &bot).await).unwrap_or_default();
                } else {
                    bot.send_message(msg.chat.id, "DEBUG: No voice but voice?")
                        .await?;
                    return Ok(());
                }

                let prompt_llm_answer = match llm_clone.lock().await.send_prompt(&my_prompt).await {
                    Ok(res) => res,
                    Err(e) => {
                        format!("Something bad happened connecting to Ambrosio LLM. Error: {e}")
                    }
                };

                if let Ok(cmd) = Command::from_str(&prompt_llm_answer) {
                    match Self::execute(&context_clone, &cmd).await {
                        Ok(output) => output,
                        Err(e) => {
                            debug!("Error executing command: {e}");
                            format!("Error: {e}")
                        }
                    };
                    bot.send_message(CHAT_ID, format!("Ok, doing {prompt_llm_answer}"))
                        .await?;
                    let _ = Self::execute(&context_clone, &cmd).await;
                } else {
                    bot.send_message(CHAT_ID, prompt_llm_answer).await?;
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
