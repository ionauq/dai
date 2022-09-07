mod huawei_config;
mod message;

use crate::common::request::{get_token_path, save_token_info, RequestToken};
use crate::common::ChannelType;
use crate::huawei::message::RequestMessage;
pub use huawei_config::HuaweiConfig;
use log::{debug, info};
pub use message::RequestMessageBuilder;
use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;

const HUAWEI_CREDENTIAL_URL: &str = "https://oauth-login.cloud.huawei.com/oauth2/v3/token";

const HUAWEI_NOTIFICATION_URL: &str = "https://push-api.cloud.huawei.com/v1/{APP_ID}/messages:send";

pub async fn send_message(
    config: &HuaweiConfig,
    message: &RequestMessage,
) -> Result<String, Box<dyn std::error::Error>> {
    let token = get_token(config).await?;
    let client = reqwest::Client::new();
    let url = HUAWEI_NOTIFICATION_URL.replace("{APP_ID}", &config.app_id.to_string());
    let res = client
        .post(&url)
        .bearer_auth(&token)
        .json(&message)
        .send()
        .await?;
    let result = res.text().await?;
    info!("【huawei】request url: \r\n{}", &url);
    debug!("【huawei】request token: \r\n{}", &token);
    info!(
        "【huawei】request body:\r\n{}",
        serde_json::to_string_pretty(&message).unwrap()
    );
    info!("【huawei】response data:\r\n{}", &result);
    Ok(result)
}

/// 获取鉴权信息 - access_token
async fn get_token(config: &HuaweiConfig) -> Result<String, Box<dyn std::error::Error>> {
    let huawei_token_path = get_token_path(config.app_id.to_string(), ChannelType::Huawei);
    let token_with_expires: String = fs::read_to_string(&huawei_token_path)?.parse()?;
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
                    &huawei_token_path,
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
                &huawei_token_path,
            )
            .await?;
            Ok(request_token.access_token)
        }
    }
}

async fn request_token(config: &HuaweiConfig) -> Result<RequestToken, Box<dyn std::error::Error>> {
    let form = [
        ("grant_type", "client_credentials"),
        ("client_id", config.client_id.as_str()),
        ("client_secret", config.client_secret.as_str()),
    ];
    let client = reqwest::Client::new();
    let result = client
        .post(HUAWEI_CREDENTIAL_URL)
        .form(&form)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    Ok(RequestToken {
        access_token: result.get("access_token").cloned().unwrap(),
        expires_in: result.get("expires_in").cloned().unwrap().parse().unwrap(),
    })
}
