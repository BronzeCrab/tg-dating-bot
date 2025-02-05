use dotenv::dotenv;
use rusqlite::{Connection, Error, Result, Statement};
use std::env;
use teloxide::prelude::*;

mod db_operations;
use db_operations::{try_to_create_db_tables, try_to_insert_user_data};
mod search;
use search::{compute_idf, compute_tf};
mod utils;

#[tokio::main]
async fn main() {
    let req_word: &str = "this";
    let words: [&str; 5] = ["this  ", "is", "a  ", "a  ", "sample"];
    let words2: [&str; 7] = [
        "this", "is", "another", "another", "example", "example", "example",
    ];
    let docs: [&[&str]; 2] = [&words, &words2];

    let term_freq: f32 = compute_tf(req_word, &words);
    println!("1 term_freq is {term_freq}");

    let term_freq: f32 = compute_tf(req_word, &words2);
    println!("2 term_freq is {term_freq}");

    let term: &str = "example";
    let idf: f32 = compute_idf(term, &docs);
    println!("idf of {term} is {idf}");

    dotenv().ok();
    let tg_bot_token: String = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    println!("The bot token is {tg_bot_token}");

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let path_to_db_file: &str = "users.db";
    let conn: Connection = Connection::open(path_to_db_file).unwrap();
    println!("{:?}", conn);

    match try_to_create_db_tables(&conn) {
        Ok(res) => {
            println!("INFO: create db res: {:?}", res);
            match try_to_insert_user_data(&conn, "afoobar", "sport123, games, music") {
                Ok(res) => println!("INFO: insert data res: {:?}", res),
                Err(error) => println!("ERROR: insert data: {:?}", error),
            };
        }
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
