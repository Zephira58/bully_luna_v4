#![allow(clippy::needless_return)]
pub fn get_insult() -> String {
    //View more details on how this works here (https://www.reddit.com/r/learnrust/comments/wz9flc/how_to_get_request_to_return_as_a_string/)
    let x = reqwest::blocking::get("https://insult.mattbas.org/api/insult")
        .expect("Get failed")
        .text()
        .expect("Couldn't get response body");
    return x;
}

pub fn send_message(insult: String) {
    //TODO: Use the discord API to send an insult to luna
    
    println!("{}", insult);
}