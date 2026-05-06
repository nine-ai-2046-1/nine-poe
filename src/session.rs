use std::fs;
use std::path::PathBuf;
use dirs::home_dir;
use mime_guess;
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json;

const CONFIG_DIR: &str = ".nine-poe";
const SESSIONS_DIR: &str = "sessions";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Content {
    Text(String),
    Parts(Vec<ContentPart>),
}

impl Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Content::Text(s) => serializer.serialize_str(s),
            Content::Parts(parts) => parts.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Content {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum ContentHelper {
            Text(String),
            Parts(Vec<ContentPart>),
        }
        
        match ContentHelper::deserialize(deserializer)? {
            ContentHelper::Text(s) => Ok(Content::Text(s)),
            ContentHelper::Parts(parts) => Ok(Content::Parts(parts)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: Content,
}

#[derive(Debug)]
pub enum SessionError {
    HomeDirNotFound,
    FailedToCreateDir(std::io::Error),
    FailedToReadSession(std::io::Error),
    FailedToWriteSession(std::io::Error),
    FailedToParseJson(serde_json::Error),
    FailedToSerializeJson(serde_json::Error),
    FailedToReadFile(std::io::Error),
    FailedToEncodeBase64,
}

pub fn normalize_session_name(name: &str) -> String {
    let mut normalized = name.replace(' ', "_");
    normalized.retain(|c| c.is_alphanumeric() || c == '_');
    normalized
}

pub fn get_sessions_dir() -> Result<PathBuf, SessionError> {
    let home = home_dir().ok_or(SessionError::HomeDirNotFound)?;
    let dir = home.join(CONFIG_DIR).join(SESSIONS_DIR);
    
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .map_err(SessionError::FailedToCreateDir)?;
    }
    
    Ok(dir)
}

pub fn get_session_path(session_name: &str) -> Result<PathBuf, SessionError> {
    let dir = get_sessions_dir()?;
    let normalized = normalize_session_name(session_name);
    Ok(dir.join(format!("{}.json", normalized)))
}

pub fn load_session(session_name: &str) -> Result<Vec<Message>, SessionError> {
    let path = get_session_path(session_name)?;
    
    if !path.exists() {
        return Ok(Vec::new());
    }
    
    let content = fs::read_to_string(&path)
        .map_err(SessionError::FailedToReadSession)?;
    
    if content.trim().is_empty() {
        return Ok(Vec::new());
    }
    
    serde_json::from_str(&content)
        .map_err(SessionError::FailedToParseJson)
}

pub fn save_session(session_name: &str, messages: &[Message]) -> Result<(), SessionError> {
    let path = get_session_path(session_name)?;
    let json = serde_json::to_string_pretty(messages)
        .map_err(SessionError::FailedToSerializeJson)?;
    
    fs::write(&path, json)
        .map_err(SessionError::FailedToWriteSession)
}

pub fn encode_file_to_data_uri(file_path: &str) -> Result<String, SessionError> {
    let bytes = fs::read(file_path)
        .map_err(SessionError::FailedToReadFile)?;
    
    let mime_type = mime_guess::from_path(file_path)
        .first_or_octet_stream();
    
    let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{};base64,{}", mime_type, encoded))
}
