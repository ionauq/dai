use crate::common::ChannelType;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::PathBuf;
use std::{fs, io, time::SystemTime};

pub const TOKEN_PATH: &str = ".dai/token/";

/// 鉴权信息
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestToken {
    pub access_token: String,
    pub expires_in: u32,
}

pub fn get_token_path(app_id: String, channel: ChannelType) -> PathBuf {
    let token_home = dirs::home_dir().unwrap().join(TOKEN_PATH);
    if !token_home.exists() {
        fs::create_dir_all(&token_home).unwrap();
    }

    let file_name = format!("{}_{}", app_id, channel);
    let token_path = &token_home.join(file_name);
    if !token_path.exists() {
        File::create(&token_path).unwrap();
    }

    token_path.clone()
}

/// 存储鉴权信息
pub async fn save_token_info(
    access_token: &str,
    expires_in: u32,
    token_path: &PathBuf,
) -> io::Result<()> {
    fs::write(
        &token_path,
        format!(
            "{},{}",
            access_token,
            expires_in as u128 * 1000
                + SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
        ),
    )
}
