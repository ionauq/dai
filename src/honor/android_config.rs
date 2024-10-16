#![allow(non_snake_case)]

use super::android_notification::{AndroidNotification, AndroidNotificationBuilder};
use serde::{Deserialize, Serialize};

/// Android通知栏消息结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct AndroidConfig {
    /// 消息缓存时间，单位是秒。在用户设备离线时，消息在Push服务器进行缓存，在消息缓存时间内用户设备上线后消息会下发，超过缓存时间后消息会丢弃。
    /// 默认值为 `86400s`（1天），最大值为 `1296000s`（15天）
    pub ttl: String,
    /// 批量任务消息标识，消息回执时会返回给应用服务器，应用服务器可以识别biTag对消息的下发情况进行统计分析
    #[serde(rename(serialize = "biTag", deserialize = "biTag"))]
    pub bi_tag: String,
    /// 自定义消息负载，此处如果设置了data，则会覆盖`message.data`字段。
    pub data: String,
    /// Android通知栏消息结构体，具体字段请参见[`AndroidNotification`]结构体的定义。
    pub notification: AndroidNotification,
}

pub struct AndroidConfigBuilder {
    ttl: String,
    bi_tag: String,
    data: String,
    notification: AndroidNotification,
}

impl AndroidConfigBuilder {
    pub fn new() -> AndroidConfigBuilder {
        AndroidConfigBuilder {
            ttl: "".to_string(),
            bi_tag: "".to_string(),
            data: "".to_string(),
            notification: AndroidNotificationBuilder::new().build(),
        }
    }

    #[allow(dead_code)]
    pub fn ttl(mut self, ttl: String) -> AndroidConfigBuilder {
        self.ttl = ttl;
        self
    }

    #[allow(dead_code)]
    pub fn bi_tag(mut self, bi_tag: String) -> AndroidConfigBuilder {
        self.bi_tag = bi_tag;
        self
    }

    #[allow(dead_code)]
    pub fn data(mut self, data: String) -> AndroidConfigBuilder {
        self.data = data;
        self
    }
    #[allow(dead_code)]
    pub fn notification(mut self, notification: AndroidNotification) -> AndroidConfigBuilder {
        self.notification = notification;
        self
    }

    pub fn build(self) -> AndroidConfig {
        AndroidConfig {
            ttl: self.ttl,
            bi_tag: self.bi_tag,
            data: self.data,
            notification: self.notification,
        }
    }
}
