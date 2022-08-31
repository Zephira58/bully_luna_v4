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

#[tokio::main]
pub async fn send_message(msg: &str) {
    let WEBHOOK_URL = dotenv::var("WEBHOOK_URL").expect("WEBHOOK_URL not found");
    let IMAGE_URL = dotenv::var("IMAGE_URL").expect("IMAGE_URL not found");
    let DISCORD_ID = dotenv::var("DISCORD_ID").expect("DISCORD_ID not found");

    let message = msg.to_owned() + &DISCORD_ID;
    let message = message.as_str();
    let mut request_body = HashMap::new();
    request_body.insert("content", message);
    request_body.insert("username", "Xanthus");
    request_body.insert("avatar_url", &IMAGE_URL);

    reqwest::Client::new() //Compiler error "Does nothing unless you use .await or poll them"
        .post(&WEBHOOK_URL)
        .json(&request_body)
        .send()
        .await;
}
