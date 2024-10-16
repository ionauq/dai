use crate::{honor, huawei, oppo, xiaomi};
use ansi_term::Colour::Green;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use toml::de::Error;

const CONFIG_PATH: &str = ".dai";
const CONFIG_FILE_NAME: &str = "config.toml";

/// dai 配置文件结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PushConfig {
    pub active: String,
    pub configs: HashMap<String, ChannelConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelConfig {
    pub xiaomi: Option<xiaomi::XiaomiConfig>,
    pub huawei: Option<huawei::HuaweiConfig>,
    pub honor: Option<honor::HonorConfig>,
    pub oppo: Option<oppo::OppoConfig>,
    pub vivo: Option<String>,
    pub meizu: Option<String>,
}

impl PushConfig {
    /// 生成默认配置文件
    pub fn init() {
        let config = include_str!("../resources/config.toml");
        let config_path = dirs::home_dir().unwrap().join(CONFIG_PATH);
        if !config_path.exists() {
            fs::create_dir_all(&config_path).unwrap();
        }

        let current_path = &config_path.join(CONFIG_FILE_NAME);
        if !current_path.exists() {
            File::create(&current_path).unwrap();
        }

        fs::write(&current_path, config).unwrap();
        info!(
            "init default config file: {}",
            Green.paint(current_path.to_str().unwrap())
        );
    }
    /// 获取配置信息
    pub fn get() -> Result<PushConfig, Error> {
        let current_path = dirs::home_dir()
            .unwrap()
            .join(CONFIG_PATH)
            .join(CONFIG_FILE_NAME);
        let config = fs::read_to_string(current_path).unwrap();
        let push_config: Result<PushConfig, _> = toml::from_str(&config);
        push_config
    }

    /// 获取通道配置
    pub fn get_channel_configs(push_config: &PushConfig) -> Option<ChannelConfig> {
        let configs = &push_config.configs;
        if configs.contains_key(&push_config.active) {
            Some(configs[&push_config.active].clone())
        } else {
            None
        }
    }
}
