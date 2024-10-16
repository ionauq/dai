use super::android_config::AndroidConfigBuilder;
use super::notification::Notification;
use super::{android_config::AndroidConfig, notification::NotificationBuilder};
use serde::{Deserialize, Serialize};

/// 请求结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub data: String,
    pub notification: Notification,
    pub android: AndroidConfig,
    pub token: Vec<String>,
}

pub struct MessageBuilder {
    data: String,
    notification: Notification,
    android: AndroidConfig,
    token: Vec<String>,
}

impl MessageBuilder {
    pub fn new() -> MessageBuilder {
        MessageBuilder {
            data: "".to_string(),
            notification: NotificationBuilder::new().build(),
            android: AndroidConfigBuilder::new().build(),
            token: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn data(mut self, data: String) -> MessageBuilder {
        self.data = data;
        self
    }
    #[allow(dead_code)]
    pub fn notification(mut self, notification: Notification) -> MessageBuilder {
        self.notification = notification;
        self
    }
    #[allow(dead_code)]
    pub fn android(mut self, android: AndroidConfig) -> MessageBuilder {
        self.android = android;
        self
    }
    #[allow(dead_code)]
    pub fn token(mut self, token: Vec<String>) -> MessageBuilder {
        self.token = token;
        self
    }

    pub fn build(self) -> Message {
        Message {
            data: self.data,
            notification: self.notification,
            android: self.android,
            token: self.token,
        }
    }
}
