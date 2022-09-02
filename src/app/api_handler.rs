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
pub async fn send_message(msg: &str, mention: bool) {
    let WEBHOOK_URL = "https://canary.discord.com/api/webhooks/1014613740139839509/sdl_Q0YDa3Nwn9JMsyPAHwWB2AjGHz-cupexxVHYjWbmDL6MO9D3FUpCGAs65EywYnEM".to_string();
    let IMAGE_URL = "https://cdn.discordapp.com/avatars/892723824297119754/fdd67fef581729a0224f7bf9e8a52d3b.png?size=1024".to_string();
    let DISCORD_ID = " <@747638440404713582>".to_string();

    let mut message = msg.to_owned();
    if mention == true {
        message = message + &DISCORD_ID;
    }
    let message = message.as_str();

    let mut request_body = HashMap::new();
    request_body.insert("content", message);
    request_body.insert("username", "Developer Build");
    request_body.insert("avatar_url", &IMAGE_URL);

    reqwest::Client::new()
        .post(&WEBHOOK_URL)
        .json(&request_body)
        .send()
        .await;
}
