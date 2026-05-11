use std::fs;
use std::path::PathBuf;
use dirs::home_dir;
use toml::Value;

const CONFIG_DIR: &str = ".nine-poe";
const CONFIG_FILE: &str = "config.toml";
const API_KEY_FIELD: &str = "NINE_POE_API_KEY";

#[allow(dead_code)]
#[derive(Debug)]
pub enum ConfigError {
    HomeDirNotFound,
    FailedToCreateDir(std::io::Error),
    FailedToReadConfig(std::io::Error),
    FailedToWriteConfig(std::io::Error),
    FailedToParseToml(toml::de::Error),
    ApiKeyMissing,
}

pub fn get_config_path() -> Result<PathBuf, ConfigError> {
    let home = home_dir().ok_or(ConfigError::HomeDirNotFound)?;
    Ok(home.join(CONFIG_DIR).join(CONFIG_FILE))
}

pub fn init_config() -> Result<PathBuf, ConfigError> {
    let home = home_dir().ok_or(ConfigError::HomeDirNotFound)?;
    let config_dir = home.join(CONFIG_DIR);
    let config_path = config_dir.join(CONFIG_FILE);

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)
            .map_err(ConfigError::FailedToCreateDir)?;
    }

    if !config_path.exists() {
        let example_config = r#"# Nine POE CLI Configuration
# 請到 https://poe.com/api_key 獲取你嘅 API Key
NINE_POE_API_KEY = ""
"#;
        fs::write(&config_path, example_config)
            .map_err(ConfigError::FailedToWriteConfig)?;
    }

    Ok(config_path)
}

pub fn get_api_key() -> Result<String, ConfigError> {
    let config_path = init_config()?;
    
    let content = fs::read_to_string(&config_path)
        .map_err(ConfigError::FailedToReadConfig)?;
    
    let toml_value: Value = toml::from_str(&content)
        .map_err(ConfigError::FailedToParseToml)?;
    
    match toml_value.get(API_KEY_FIELD) {
        Some(Value::String(key)) if !key.trim().is_empty() => Ok(key.trim().to_string()),
        _ => Err(ConfigError::ApiKeyMissing),
    }
}
