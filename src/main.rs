extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let TELEGRAM_BOT_TOKEN: String = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    println!("The bot token is {TELEGRAM_BOT_TOKEN}");
}
