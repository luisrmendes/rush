mod office_env_fsm;
use office_env_fsm::Fsm;

pub enum Operation {
    GetIpv4,
    GetIpv6,
    WakeupDesktop,
}

pub mod commands {
    use crate::Operation;

    pub fn execute_commands(op: Operation) {
        match op {
            Operation::GetIpv4 => {}
            Operation::GetIpv6 => {}
            Operation::WakeupDesktop => {}
        }
    }
}

pub mod telegram {
    use crate::Operation;

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

    pub async fn get_telegram_task() {
        use teloxide::prelude::*;

        pretty_env_logger::init();
        log::info!("Starting throw dice bot...");

        let bot = Bot::from_env();

        return teloxide::repl(bot, |bot: Bot, msg: Message| async move {
            println!("{:?}", msg.text());

            let _command = parse_commands(msg.text());
            bot.send_dice(msg.chat.id).await?;
            Ok(())
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    let mut connection = Fsm::new();
    connection.run().await;
}
