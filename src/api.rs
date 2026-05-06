use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use crate::session::{Message, Content, ContentPart};

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
    content: Content,
}

#[derive(Debug)]
pub enum ApiError {
    RequestFailed(reqwest::Error),
    NoChoices,
    FailedToParseResponse(serde_json::Error),
    UnexpectedResponseFormat,
}

fn extract_text_from_content(content: Content) -> Option<String> {
    match content {
        Content::Text(s) => Some(s),
        Content::Parts(parts) => {
            for part in parts {
                if let ContentPart::Text { text } = part {
                    return Some(text);
                }
            }
            None
        }
    }
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
        Some(choice) => {
            extract_text_from_content(choice.message.content)
                .ok_or(ApiError::UnexpectedResponseFormat)
        }
        None => Err(ApiError::NoChoices),
    }
}
