#![allow(clippy::needless_return)]
#![allow(non_snake_case)]
#![allow(unused_must_use)]
use std::collections::HashMap;

pub fn get_insult() -> String {
    //View more details on how this works here (https://www.reddit.com/r/learnrust/comments/wz9flc/how_to_get_request_to_return_as_a_string/)
    let x = reqwest::blocking::get("https://insult.mattbas.org/api/insult")
        .expect("Get failed")
        .text()
        .expect("Couldn't get response body");
    return x;
}

mod tokens;
use tokens::*;

#[tokio::main]
pub async fn send_message(msg: &str, mention: bool) {
    let WEBHOOK_URL = WEBHOOK_URL();
    let IMAGE_URL = IMAGE_URL();
    let DISCORD_ID = DISCORD_ID();
    let USERNAME = USERNAME();

    let mut message = msg.to_owned();
    if mention == true {
        message = message + &DISCORD_ID;
    }
    let message = message.as_str();

    let mut request_body = HashMap::new();
    request_body.insert("content", message);
    request_body.insert("username", &USERNAME);
    request_body.insert("avatar_url", &IMAGE_URL);

    reqwest::Client::new()
        .post(&WEBHOOK_URL)
        .json(&request_body)
        .send()
        .await;
}
