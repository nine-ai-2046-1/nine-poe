use std::fs;
use std::path::PathBuf;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use serde_json;

const CONFIG_DIR: &str = ".nine-poe";
const SESSIONS_DIR: &str = "sessions";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug)]
pub enum SessionError {
    HomeDirNotFound,
    FailedToCreateDir(std::io::Error),
    FailedToReadSession(std::io::Error),
    FailedToWriteSession(std::io::Error),
    FailedToParseJson(serde_json::Error),
    FailedToSerializeJson(serde_json::Error),
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
