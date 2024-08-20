use crate::{
    commands::{self, Operation},
    Context,
};
use log::trace;
use std::sync::Arc;

pub struct TelegramBot {
    context: Context,
}

impl TelegramBot {
    pub fn new(ctx: Context) -> Self {
        Self { context: ctx }
    }

    async fn execute(ctx: &Context, op: &Operation) -> Result<String, String> {
        match op {
            Operation::SuspendDesktop => commands::suspend(ctx.systems[1].clone()).await,
            Operation::WakeupDesktop => commands::wakeup(ctx.systems[2].clone()).await,
            Operation::StatusDesktop => Ok(commands::is_online(&ctx.systems[2].clone()).to_string()),
            Operation::GetIpv4 => commands::get_ipv4().await,
        }
    }

    fn parse_commands(text: Option<&str>) -> Result<Operation, String> {
        let parsed_msg: &str = text.unwrap_or_default();
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

        let bot = Bot::from_env();
        let context_arc = Arc::new(self.context.clone());

        teloxide::repl(bot, move |bot: Bot, msg: Message| {
            let context_clone = Arc::clone(&context_arc);
            async move {
                trace!("Received from bot: {:?}", msg.text());

                let command = match Self::parse_commands(msg.text()) {
                    Ok(cmd) => cmd,
                    Err(e) => {
                        let reply = format!("Failed to parse command. Error: {e}");
                        trace!("{reply}");
                        bot.send_message(msg.chat.id, reply).await?;
                        return Ok(());
                    }
                };

                let cmd_output = match Self::execute(&context_clone, &command).await {
                    Ok(output) => output,
                    Err(e) => {
                        trace!("Error executing command: {e}");
                        format!("Error: {e}")
                    }
                };

                trace!("Sending to bot: {:?}", cmd_output);
                bot.send_message(msg.chat.id, cmd_output).await?;
                Ok(())
            }
        })
        .await;
    }
}
