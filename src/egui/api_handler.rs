#![allow(clippy::needless_return)]
#![allow(non_snake_case)]
use dotenv;
use std::collections::HashMap;

pub fn get_insult() -> String {
    //View more details on how this works here (https://www.reddit.com/r/learnrust/comments/wz9flc/how_to_get_request_to_return_as_a_string/)
    let x = reqwest::blocking::get("https://insult.mattbas.org/api/insult")
        .expect("Get failed")
        .text()
        .expect("Couldn't get response body");
    return x;
}

pub fn send_message(msg: &str) {
    let WEBHOOK_URL = dotenv::var("WEBHOOK_URL").unwrap();
    let IMAGE_URL = dotenv::var("IMAGE_URL").unwrap();

    let mut request_body = HashMap::new();
    request_body.insert("content", msg);
    request_body.insert("username", "Xanthus");
    request_body.insert("avatar_url", &IMAGE_URL);

    reqwest::Client::new()
        .post(&WEBHOOK_URL)
        .json(&request_body)
        .send();

    //-Debug Prints-
    //println!("Message should be sent to discord");
    //println!("{}", &WEBHOOK_URL);
    //match request_body.get("avatar_url") {
    //    Some(avatar_url) => println!("Avatar URL: {}", avatar_url),
    //    _ => println!("No avatar URL"),
    //}
    //match request_body.get("content") {
    //    Some(content) => println!("Content: {}", content),
    //    _ => println!("No content"),
    //}
    //match request_body.get("username") {
    //    Some(username) => println!("Username: {}", username),
    //    _ => println!("No username"),
    //}
}
