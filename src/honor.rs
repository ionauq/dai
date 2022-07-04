mod android_config;
mod android_notification;
mod badge_notification;
mod button;
mod click_action;
mod honor_config;
mod message;
mod notification;
mod request_token;

pub use crate::common::ChannelType;
pub use android_config::{AndroidConfig, AndroidConfigBuilder};
pub use android_notification::{AndroidNotification, AndroidNotificationBuilder};
pub use badge_notification::{BadgeNotification, BadgeNotificationBuilder};
pub use click_action::{ClickAction, ClickActionBuilder};
pub use honor_config::HonorConfig;
pub use message::{Message, MessageBuilder};
pub use notification::{Notification, NotificationBuilder};
use request_token::RequestToken;
use std::path::PathBuf;
use std::{fs, fs::File, io, time::SystemTime};

const HONOR_CREDENTIAL_URL: &str =
    "https://iam.developer.hihonor.com/auth/realms/developer/protocol/openid-connect/token";

const HONOR_NOTIFICATION_URL: &str =
    "https://push-api.cloud.hihonor.com/api/v1/{APPID}/sendMessage";

const TOKEN_PATH: &str = ".dai/token/";

/// 消息推送
pub async fn send_message(
    config: &HonorConfig,
    message: &Message,
) -> Result<String, Box<dyn std::error::Error>> {
    let token = get_token(config).await?;
    let client = reqwest::Client::new();
    let url = HONOR_NOTIFICATION_URL.replace("{APPID}", &config.app_id.to_string());
    let res = client
        .post(&url)
        .bearer_auth(&token)
        .header(
            "timestamp",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .to_string(),
        )
        .json(&message)
        .send()
        .await?;
    let result = res.text().await?;
    println!("【honor】request url: \r\n{}", &url);
    println!("【honor】request token: \r\n{}", &token);
    println!(
        "【honor】request body:\r\n{}",
        serde_json::to_string_pretty(&message).unwrap()
    );
    println!("【honor】response data:\r\n{}", &result);
    Ok(result.clone())
}

/// 获取鉴权信息 - access_token
async fn get_token(config: &HonorConfig) -> Result<String, Box<dyn std::error::Error>> {
    let token_home = dirs::home_dir().unwrap().join(TOKEN_PATH);
    if !token_home.exists() {
        fs::create_dir_all(&token_home).unwrap();
    }

    let file_name = format!("{}_{}", config.app_id, ChannelType::Honor.to_string());
    let honor_token_path = &token_home.join(file_name);
    if !honor_token_path.exists() {
        File::create(&honor_token_path).unwrap();
    }

    let token_with_expires: String = fs::read_to_string(&honor_token_path)?.parse()?;
    match token_with_expires.split_once(',') {
        Some((token, expires)) => {
            if SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                >= expires.parse::<u128>().unwrap()
            {
                let request_token = request_token(config).await?;
                save_token_info(&request_token, &honor_token_path).await?;
                Ok(request_token.access_token)
            } else {
                Ok(token.to_string())
            }
        }
        _ => {
            let request_token = request_token(config).await?;
            save_token_info(&request_token, &honor_token_path).await?;
            Ok(request_token.access_token)
        }
    }
}

/// 请求鉴权信息
async fn request_token(config: &HonorConfig) -> Result<RequestToken, Box<dyn std::error::Error>> {
    let form = [
        ("grant_type", "client_credentials"),
        ("client_id", config.client_id.as_str()),
        ("client_secret", config.client_secret.as_str()),
    ];
    let client = reqwest::Client::new();
    let token = client
        .post(HONOR_CREDENTIAL_URL)
        .form(&form)
        .send()
        .await?
        .json::<RequestToken>()
        .await?;
    Ok(token)
}

async fn save_token_info(
    request_token: &RequestToken,
    honor_token_path: &PathBuf,
) -> io::Result<()> {
    fs::write(
        &honor_token_path,
        format!(
            "{},{}",
            request_token.access_token,
            request_token.expires_in as u128 * 1000
                + SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis()
        ),
    )
}
