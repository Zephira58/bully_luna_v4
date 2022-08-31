#![allow(clippy::needless_return)]

pub fn get_insult() -> String {
    //View more details on how this works here (https://www.reddit.com/r/learnrust/comments/wz9flc/how_to_get_request_to_return_as_a_string/)
    let x = reqwest::blocking::get("https://insult.mattbas.org/api/insult")
        .expect("Get failed")
        .text()
        .expect("Couldn't get response body");
    return x;
}

use webhook::client::{WebhookClient, WebhookResult};
const IMAGE_URL: &'static str = "https://cdn.discordapp.com/avatars/292971545956188160/eab559efa07f0f3dd13d21ac5f26c4ce.png?size=1024";
pub async fn send_message(msg: &str) -> WebhookResult<()> {
    let url = "https://canary.discord.com/api/webhooks/1014531600547074070/IAmOcsyW0QjAAsM6gKdVNrjgOxLoUZAjakURSjWjbuuC61GlJDL5cDxgobjP-kqTDosH";
    let client = WebhookClient::new(&url);
    let webhook_info = client.get_information().await?;
    println!("webhook: {:?}", webhook_info);

    client
        .send(|message| {
            message
                .content(msg)
                .username("Xanthus")
                .avatar_url(IMAGE_URL)
        })
        .await?;

    Ok(())
}
