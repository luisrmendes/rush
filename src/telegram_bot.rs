use std::sync::Arc;

use crate::{
    commands::{self, Operation},
    Context,
};

pub struct TelegramBot {
    context: Context,
}

impl TelegramBot {
    pub fn new(ctx: Context) -> Self {
        Self { context: ctx }
    }

    async fn execute(ctx: &Context, op: &Operation) -> Result<String, String> {
        match op {
            Operation::GetIpv4 => {
                let output = match commands::get_ipv4(ctx.systems[0].clone()).await {
                    Ok(output) => Ok(output),
                    Err(e) => {
                        return Err(format!("Error: {e}"));
                    }
                };
                output
            }

            Operation::GetIpv6 | Operation::WakeupDesktop => {
                return Err("Not implemented".to_owned());
            }
        }
    }

    fn parse_commands(text: Option<&str>) -> Option<Operation> {
        let parsed_msg: &str = match text {
            Some(msg) => msg,
            None => "",
        };
        match parsed_msg {
            "/ipv4" => Some(Operation::GetIpv4),
            "/ipv6" => Some(Operation::GetIpv6),
            "/desktop_wakeup" => Some(Operation::WakeupDesktop),
            _ => None,
        }
    }

    pub async fn run(&self) {
        use teloxide::prelude::*;

        pretty_env_logger::init();
        let bot = Bot::from_env();
        let context_arc = Arc::new(self.context.clone());

        teloxide::repl(bot, move |bot: Bot, msg: Message| {
            let context_clone = Arc::clone(&context_arc);
            async move {
                println!("{:?}", msg.text());

                let command = match Self::parse_commands(msg.text()) {
                    Some(cmd) => cmd,
                    None => {
                        println!("Failed to parse command");
                        return Ok(());
                    }
                };

                let cmd_output = match Self::execute(&*context_clone, &command).await {
                    Ok(output) => output,
                    Err(e) => {
                        println!("Error executing command: {e}");
                        return Ok(());
                    }
                };

                bot.send_message(msg.chat.id, cmd_output).await?;
                Ok(())
            }
        })
        .await;
    }
}
