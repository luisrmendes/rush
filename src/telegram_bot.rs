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
            Operation::SuspendDesktop => {
                let output = match commands::suspend(ctx.systems[1].clone()).await {
                    Ok(output) => Ok(output),
                    Err(e) => {
                        return Err(format!("Error: {e}"));
                    }
                };
                output
            }
            Operation::GetIpv4 => {
                let output = match commands::get_ipv4(ctx.systems[0].clone()).await {
                    Ok(output) => Ok(output),
                    Err(e) => {
                        return Err(format!("Error: {e}"));
                    }
                };
                output
            }
            Operation::WakeupDesktop => {
                let output = match commands::wakeup(ctx.systems[1].clone()).await {
                    Ok(output) => Ok(output),
                    Err(e) => {
                        return Err(format!("Error: {e}"));
                    }
                };
                output
            }
            Operation::StatusDesktop => {
                return Ok(commands::is_online(ctx.systems[1].clone())
                    .await
                    .to_string());
            }
        }
    }

    fn parse_commands(text: Option<&str>) -> Result<Operation, String> {
        let parsed_msg: &str = match text {
            Some(msg) => msg,
            None => "",
        };
        match parsed_msg {
            "/ipv4" => Ok(Operation::GetIpv4),
            "/desktop_wakeup" => Ok(Operation::WakeupDesktop),
            "/desktop_status" => Ok(Operation::StatusDesktop),
            "/desktop_suspend" => Ok(Operation::SuspendDesktop),
            other => Err(format!("Unknown command {other}")),
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
                    Ok(cmd) => cmd,
                    Err(e) => {
                        let reply = format!("Failed to parse command. Error: {e}");
                        println!("{reply}");
                        bot.send_message(msg.chat.id, reply).await?;
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
