use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use crate::session::Message;

const API_URL: &str = "https://api.poe.com/v1/chat/completions";
const ANTHROPIC_VERSION: &str = "2023-06-01";

#[derive(Serialize)]
struct Request {
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Debug)]
pub enum ApiError {
    RequestFailed(reqwest::Error),
    NoChoices,
    FailedToParseResponse(serde_json::Error),
}

pub fn send_message(
    api_key: &str,
    model: &str,
    messages: Vec<Message>,
) -> Result<String, ApiError> {
    let client = Client::new();
    let request_body = Request {
        model: model.to_string(),
        messages,
    };
    
    let response = client
        .post(API_URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("anthropic-version", ANTHROPIC_VERSION)
        .json(&request_body)
        .send()
        .map_err(ApiError::RequestFailed)?;
    
    let response_body: Response = response
        .json()
        .map_err(ApiError::RequestFailed)?;
    
    match response_body.choices.into_iter().next() {
        Some(choice) => Ok(choice.message.content),
        None => Err(ApiError::NoChoices),
    }
}
