use dotenv::dotenv;
use rusqlite::{Connection, Error, Result, Statement};
use std::env;
use teloxide::prelude::*;

mod db_operations;
use db_operations::{
    get_tokens_by_user_id, try_to_create_db_tables, try_to_insert_user_data,
    try_to_insert_user_token_relations, try_to_insert_user_tokens,
};
mod search;
use search::{compute_idf, compute_tf};
mod utils;
use utils::split_into_tokens;

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
            for i in 1..3 {
                let description: &str;
                let tg_username: &str;
                if i == 1 {
                    description = "sport123,games,music,MUSIC";
                    tg_username = "test1";
                }
                else {
                    description = "sport123,alko,grugs";
                    tg_username = "test2";
                }
                match try_to_insert_user_data(&conn, tg_username, description) {
                    Ok(user_id) => {
                        println!("INFO: insert data res user_id: {:?}", user_id);
                        let tokens: Vec<String> = split_into_tokens(description);
                        println!("Split into tokens: {:?}", tokens);
                        let tokens_ids: Vec<u32> = try_to_insert_user_tokens(&conn, tokens).unwrap();
                        println!("Tokens_ids: {:?}", tokens_ids);
                        try_to_insert_user_token_relations(&conn, user_id, tokens_ids);
                        println!("Get user tokens by user_id");
                        let user_tokens = get_tokens_by_user_id(&conn, user_id);
                        println!("Token names by user_id {user_id}: {:?}", user_tokens);
                    }
                    Err(error) => println!("ERROR: insert data: {:?}", error),
                };
            };
        }
        Err(error) => println!("ERROR: create db: {:?}", error),
    };

    // let bot = Bot::new(tg_bot_token);

    // teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    //     println!("Recieve msg {:?}", msg);
    //     bot.send_dice(msg.chat.id).await?;
    //     Ok(())
    // })
    // .await;
}
