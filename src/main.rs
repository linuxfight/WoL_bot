use std::time::Duration;
use teloxide::{prelude::*, utils::command::BotCommands};
use wakey::WolPacket;
use ping::ping;
use rand::random;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    bot_token: String,
    ip: String,
    mac: String,
    admin_id: i64,
}

fn read_config_from_file(file_path: &str) -> Config {
    use std::fs::File;
    use std::io::prelude::*;

    // Read the contents of the file
    let mut file = File::open(file_path).expect("Unable to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read config file");

    // Parse the JSON contents into Config struct
    match serde_json::from_str(&contents) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error parsing config file: {}", err);
            std::process::exit(1);  // Exit the program if parsing fails
        }
    }
}

lazy_static! {
    static ref CONFIG: Config = read_config_from_file("config.json");
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting wol bot...");

    let bot = Bot::new(&CONFIG.bot_token);

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "start this bot")]
    Start,
    #[command(description = "display this text.")]
    Help,
    #[command(description = "status of pc")]
    PC,
    #[command(description = "power on pc")]
    Power
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    if &msg.chat.id.0 != &CONFIG.admin_id {
        bot.send_message(msg.chat.id, "You're not allowed to use this bot").await?;
    } else {
        match cmd {
            Command::Start => {
                bot.send_message(msg.chat.id, "Hi, I'm your personal WoL bot").await?;
                let commands = Command::bot_commands();
                bot.set_my_commands(commands).await?;
            },
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
            },
            Command::PC => {
                let ip = &CONFIG.ip;
                let addr = ip.parse().expect("IP is invalid");
                let timeout = Duration::from_secs(1);
                let resp = ping(
                    addr,
                    Some(timeout),
                    Some(166),
                    Some(3),
                    Some(5),
                    Some(&random()),
                ).is_ok();
                if resp {
                    bot.send_message(msg.chat.id, "PC is on").await?;
                } else {
                    bot.send_message(msg.chat.id, "PC is off").await?;
                }
            }
            Command::Power => {
                let wol = WolPacket::from_string(&CONFIG.mac, ':').unwrap();
                if wol.send_magic().is_ok() {
                    bot.send_message(msg.chat.id, "Sent the magic packet").await?;
                } else {
                    bot.send_message(msg.chat.id, "Failed to send the magic packet").await?;
                }
            }
        }
    }
    Ok(())
}
