#![allow(clippy::needless_return)]
#![allow(non_snake_case)]
#![allow(unused_must_use)]
use std::collections::HashMap;

pub fn get_insult() -> poll_promise::Promise<Option<String>> {
    let (sender, promise) = poll_promise::Promise::new();
    ehttp::fetch(
        ehttp::Request::get("https://insult.mattbas.org/api/insult"),
        move |response| {
            let response = response.ok(); // convert to Option
            let text = response.and_then(|response| response.text().map(|text| text.to_string()));
            sender.send(text);
        },
    );
    promise
}

pub fn send_message(
    msg: &str,
    mention: bool,
) -> poll_promise::Promise<ehttp::Result<ehttp::Response>> {
    let WEBHOOK_URL = "https://canary.discord.com/api/webhooks/1014613740139839509/sdl_Q0YDa3Nwn9JMsyPAHwWB2AjGHz-cupexxVHYjWbmDL6MO9D3FUpCGAs65EywYnEM".to_string();
    let IMAGE_URL = "https://cdn.discordapp.com/avatars/892723824297119754/fdd67fef581729a0224f7bf9e8a52d3b.png?size=1024".to_string();
    let DISCORD_ID = " <@747638440404713582>".to_string();

    let mut message = msg.to_owned();
    if mention {
        message = message + &DISCORD_ID;
    }
    let message = message.as_str();

    let mut request_body = HashMap::new();

    request_body.insert("content", message);
    request_body.insert("username", "Developer Build");
    request_body.insert("avatar_url", &IMAGE_URL);

    let json = serde_json::to_vec(&request_body).expect("Failed to generate request json body");

    let request = ehttp::Request::post(&WEBHOOK_URL, json);
    let (sender, promise) = poll_promise::Promise::new();
    ehttp::fetch(request, move |response| {
        tracing::info!("Discord api response: {:?}", response);
        sender.send(response);
    });
    promise
}
