use clap::Parser;
use crate::config::{ConfigError, get_api_key};
use crate::session::{load_session, save_session, Message};
use crate::api::send_message;

mod config;
mod session;
mod api;

#[derive(Parser)]
#[command(name = "nine-poe", version = "0.1.0", about = "POE API CLI 工具")]
struct Cli {
    #[arg(short, long, help = "模型名稱")]
    model: String,
    
    #[arg(short, long, help = "用戶消息")]
    prompt: String,
    
    #[arg(short, long, help = "會話名稱（可選）")]
    session: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    
    let api_key = match get_api_key() {
        Ok(key) => key,
        Err(ConfigError::ApiKeyMissing) => {
            eprintln!("錯誤：未搵到 NINE_POE_API_KEY，請更新 ~/.nine-poe/config.toml");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("讀取配置失敗：{:?}", e);
            std::process::exit(1);
        }
    };
    
    let mut messages = if let Some(ref session_name) = cli.session {
        match load_session(session_name) {
            Ok(history) => history,
            Err(e) => {
                eprintln!("加載會話失敗：{:?}", e);
                std::process::exit(1);
            }
        }
    } else {
        Vec::new()
    };
    
    let user_message = Message {
        role: "user".to_string(),
        content: cli.prompt.clone(),
    };
    messages.push(user_message.clone());
    
    match send_message(&api_key, &cli.model, messages.clone()) {
        Ok(response) => {
            println!("{}", response);
            
            if let Some(ref session_name) = cli.session {
                let assistant_message = Message {
                    role: "assistant".to_string(),
                    content: response,
                };
                messages.push(assistant_message);
                
                if let Err(e) = save_session(session_name, &messages) {
                    eprintln!("保存會話失敗：{:?}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("API 請求失敗：{:?}", e);
            std::process::exit(1);
        }
    }
}
