mod message;
mod xiaomi_config;

use crate::xiaomi::message::Message;
use log::{error, info};
pub use message::MessageBuilder;
pub use xiaomi_config::XiaomiConfig;

const XIAOMI_NOTIFICATION_URL_REG_ID: &str = "https://api.xmpush.xiaomi.com/v3/message/regid";

const XIAOMI_NOTIFICATION_URL_ALIAS: &str = "https://api.xmpush.xiaomi.com/v3/message/alias";

const XIAOMI_NOTIFICATION_URL_USER_ACCOUNT: &str =
    "https://api.xmpush.xiaomi.com/v2/message/user_account";

/// 小米通道下发消息
pub async fn send_message(
    config: &XiaomiConfig,
    message: &Message,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut url = "".to_string();
    if !message.registration_id.is_empty() {
        url = XIAOMI_NOTIFICATION_URL_REG_ID.to_string();
    } else if !message.alias.is_empty() {
        url = XIAOMI_NOTIFICATION_URL_ALIAS.to_string();
    } else if !message.user_account.is_empty() {
        url = XIAOMI_NOTIFICATION_URL_USER_ACCOUNT.to_string()
    }

    if url.is_empty() {
        error!("请填写registration_id、alias或者user_account参数");
        Err("")?
    }

    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .bearer_auth(&config.app_secret)
        .json(&message)
        .send()
        .await?;
    let result = res.text().await?;
    info!("【xiaomi】request url: {}", &url);
    info!(
        "【xiaomi】request body:\r\n{}",
        serde_json::to_string_pretty(&message).unwrap()
    );
    info!("【xiaomi】response data:\r\n{}", &result);
    Ok(result)
}
