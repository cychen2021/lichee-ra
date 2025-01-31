use serde::{Deserialize, Serialize};
use toml;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct AIService {
    provider: String,
    #[serde(flatten)]
    provider_config: toml::Value,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(rename = "ai-service")]
    ai_service: AIService,
}

#[tauri::command]
pub fn load_config(config_path: String) -> Config {
    let config_str = fs::read_to_string(config_path)
        .expect("Failed to read config file");
    
    let config: Config = toml::from_str(&config_str)
        .expect("Failed to parse TOML");
        
    config
} 

#[tauri::command]
pub fn save_config(config: Config, config_path: String) {
    let config_str = toml::to_string(&config)
        .expect("Failed to serialize config to TOML");
    
    fs::write(config_path, config_str)
        .expect("Failed to write config file");
}

