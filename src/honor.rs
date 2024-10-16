mod android_config;
mod android_notification;
mod badge_notification;
mod button;
mod click_action;
mod honor_config;
mod message;
mod notification;

use crate::common::request::{get_token_path, save_token_info, RequestToken};
use crate::common::transmit::Transmit;
pub use crate::common::ChannelType;
pub use android_config::{AndroidConfig, AndroidConfigBuilder};
pub use android_notification::{AndroidNotification, AndroidNotificationBuilder};
pub use badge_notification::{BadgeNotification, BadgeNotificationBuilder};
pub use click_action::{ClickAction, ClickActionBuilder};
pub use honor_config::HonorConfig;
use log::{debug, info};
pub use message::{Message, MessageBuilder};
pub use notification::{Notification, NotificationBuilder};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::{fs, time::SystemTime};

const HONOR_CREDENTIAL_URL: &str =
    "https://iam.developer.hihonor.com/auth/realms/developer/protocol/openid-connect/token";

const HONOR_NOTIFICATION_URL: &str =
    "https://push-api.cloud.hihonor.com/api/v1/{APP_ID}/sendMessage";

impl Transmit for Message {
    fn init_message_file(&self) {
        todo!()
    }

    fn send(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

/// 消息推送
pub async fn send_message(
    config: &HonorConfig,
    message: &Message,
) -> Result<String, Box<dyn Error>> {
    let token = get_token(config).await?;
    let client = reqwest::Client::new();
    let url = HONOR_NOTIFICATION_URL.replace("{APP_ID}", &config.app_id.to_string());
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
    info!("【honor】request url: \r\n{}", &url);
    debug!("【honor】request token: \r\n{}", &token);
    info!(
        "【honor】request body:\r\n{}",
        serde_json::to_string_pretty(&message).unwrap()
    );
    info!("【honor】response data:\r\n {}", &result);
    Ok(result)
}

/// 获取鉴权信息 - access_token
async fn get_token(config: &HonorConfig) -> Result<String, Box<dyn Error>> {
    let honor_token_path = get_token_path(config.app_id.to_string(), ChannelType::Honor);
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
                save_token_info(
                    &request_token.access_token,
                    request_token.expires_in,
                    &honor_token_path,
                )
                .await?;
                Ok(request_token.access_token)
            } else {
                Ok(token.to_string())
            }
        }
        _ => {
            let request_token = request_token(config).await?;
            save_token_info(
                &request_token.access_token,
                request_token.expires_in,
                &honor_token_path,
            )
            .await?;
            Ok(request_token.access_token)
        }
    }
}

/// 请求鉴权信息
async fn request_token(config: &HonorConfig) -> Result<RequestToken, Box<dyn Error>> {
    let form = [
        ("grant_type", "client_credentials"),
        ("client_id", config.client_id.as_str()),
        ("client_secret", config.client_secret.as_str()),
    ];
    let client = reqwest::Client::new();
    let result = client
        .post(HONOR_CREDENTIAL_URL)
        .form(&form)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?;

    Ok(RequestToken {
        access_token: result
            .get("access_token")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string(),
        expires_in: result.get("expires_in").unwrap().as_u64().unwrap() as u32,
    })
}
