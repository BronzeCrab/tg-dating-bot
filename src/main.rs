use dotenv::dotenv;
use rusqlite::{Connection, Error, Result, Statement};
use std::env;
use teloxide::prelude::*;

mod db_operations;
use db_operations::try_to_create_db_tables;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let tg_bot_token: String = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    println!("The bot token is {tg_bot_token}");

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let path_to_db_file: &str = "users.db";
    let conn: Connection = Connection::open(path_to_db_file).unwrap();
    println!("{:?}", conn);

    match try_to_create_db_tables(&conn) {
        Ok(res) => println!("INFO: create db res: {:?}", res),
        Err(error) => println!("ERROR: create db: {:?}", error),
    };

    let bot = Bot::new(tg_bot_token);

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        println!("Recieve msg {:?}", msg);
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
