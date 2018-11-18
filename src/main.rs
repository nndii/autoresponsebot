extern crate autoresponsebot;
extern crate dotenv;

use autoresponsebot::{load_rules, run};
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let token = env::var("AUTORESPONSEBOT_TOKEN").expect("Can not to get token");
    let rules_path = env::var("AUTORESPONSEBOT_RULES").expect("Can not to get rules path");
    let chat_id = env::var("AUTORESPONSEBOT_CHAT_ID").expect("Can not get chat id");
    let chat_id = chat_id.parse::<i64>().expect("Invalid chat ID");
    let rules = load_rules(rules_path).expect("Failed to load rules");
    run(token, chat_id, rules);
}
