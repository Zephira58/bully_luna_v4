#![allow(clippy::needless_return)]
#![allow(non_snake_case)]
use dotenv;

pub fn get_insult() -> String {
    //View more details on how this works here (https://www.reddit.com/r/learnrust/comments/wz9flc/how_to_get_request_to_return_as_a_string/)
    let x = reqwest::blocking::get("https://insult.mattbas.org/api/insult")
        .expect("Get failed")
        .text()
        .expect("Couldn't get response body");
    return x;
}

use webhook::client::{WebhookClient, WebhookResult};

pub async fn send_message(msg: &str) -> WebhookResult<()> {
    let IMAGE_URL = dotenv::var("IMAGE_URL").unwrap();
    let url = dotenv::var("WEBHOOK_URL").unwrap();
    let client = WebhookClient::new(&url);
    let webhook_info = client.get_information().await?;
    println!("webhook: {:?}", webhook_info);

    client
        .send(|message| {
            message
                .content(msg)
                .username("Xanthus")
                .avatar_url(&IMAGE_URL)
        })
        .await?;

    Ok(())
}
