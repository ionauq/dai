use crate::honor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;

const CONFIG_PATH: &str = ".dai";
const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PushConfig {
    active: String,
    configs: HashMap<String, ChannelConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelConfig {
    pub xiaomi: Option<String>,
    pub huawei: Option<String>,
    pub honor: Option<honor::HonorConfig>,
    pub vivo: Option<String>,
    pub oppo: Option<String>,
    pub meizu: Option<String>,
}

impl PushConfig {
    /// 生成默认应用通道配置文件
    pub fn init() {
        let config = include_str!("resources/config.toml");
        let config_path = dirs::home_dir().unwrap().join(CONFIG_PATH);
        if !config_path.exists() {
            fs::create_dir_all(&config_path).unwrap();
        }

        let current_path = &config_path.join(CONFIG_FILE_NAME);
        if !current_path.exists() {
            File::create(&current_path).unwrap();
        }

        fs::write(&current_path, config).unwrap();
        println!("init config file: {}", &current_path.to_str().unwrap());
    }

    /// 获取指定应用的通道信息
    pub fn get_app_config() -> Option<ChannelConfig> {
        let current_path = dirs::home_dir()
            .unwrap()
            .join(CONFIG_PATH)
            .join(CONFIG_FILE_NAME);
        let config = fs::read_to_string(current_path).unwrap();
        let push_config: PushConfig = toml::from_str(&config).unwrap();
        let configs = push_config.configs;
        if configs.contains_key(&push_config.active) {
            Some(configs[&push_config.active].clone())
        } else {
            None
        }
    }
}
