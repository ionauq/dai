#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// 小米通道消息结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub payload: String,
    pub restricted_package_name: String,
    pub pass_through: u32,
    pub title: String,
    pub description: String,
    pub notify_type: i32,
    pub time_to_live: u64,
    pub time_to_send: u64,
    pub notify_id: u32,
    pub registration_id: String,
    pub alias: String,
    pub user_account: String,

    #[serde(rename = "extra.sound_uri")]
    pub extra_sound_uri: String,

    #[serde(rename = "extra.ticker")]
    pub extra_ticker: String,

    #[serde(rename = "extra.notify_foreground")]
    pub extra_notify_foreground: String,

    #[serde(rename = "extra.notify_effect")]
    pub extra_notify_effect: String,

    #[serde(rename = "extra.intent_uri")]
    pub extra_intent_uri: String,

    #[serde(rename = "extra.web_uri")]
    pub extra_web_uri: String,

    #[serde(rename = "extra.flow_control")]
    pub extra_flow_control: i32,

    #[serde(rename = "extra.jobkey")]
    pub extra_job_key: String,

    #[serde(rename = "extra.callback")]
    pub extra_callback: String,
    #[serde(rename = "extra.callback.param")]
    pub extra_callback_param: String,

    #[serde(rename = "extra.callback.type")]
    pub extra_callback_type: u32,

    #[serde(rename = "extra.locale")]
    pub extra_locale: String,

    #[serde(rename = "extra.locale_not_in")]
    pub extra_locale_not_in: String,

    #[serde(rename = "extra.model")]
    pub extra_model: String,

    #[serde(rename = "extra.model_not_in")]
    pub extra_model_not_in: String,

    #[serde(rename = "extra.app_version")]
    pub extra_app_version: String,

    #[serde(rename = "extra.app_version_not_in")]
    pub extra_app_version_not_in: String,

    #[serde(rename = "extra.connpt")]
    pub extra_network: String,

    #[serde(rename = "extra.only_send_once")]
    pub extra_only_send_once: String,
}

pub struct MessageBuilder {
    payload: String,
    restricted_package_name: String,
    pass_through: u32,
    title: String,
    description: String,
    notify_type: i32,
    time_to_live: u64,
    time_to_send: u64,
    notify_id: u32,
    registration_id: String,
    alias: String,
    user_account: String,
    extra_sound_uri: String,
    extra_ticker: String,
    extra_notify_foreground: String,
    extra_notify_effect: String,
    extra_intent_uri: String,
    extra_web_uri: String,
    extra_flow_control: i32,
    extra_job_key: String,
    extra_callback: String,
    extra_callback_param: String,
    extra_callback_type: u32,
    extra_locale: String,
    extra_locale_not_in: String,
    extra_model: String,
    extra_model_not_in: String,
    extra_app_version: String,
    extra_app_version_not_in: String,
    extra_network: String,
    extra_only_send_once: String,
}

impl MessageBuilder {
    pub fn new() -> MessageBuilder {
        MessageBuilder {
            payload: "".to_string(),
            restricted_package_name: "".to_string(),
            pass_through: 0,
            title: "".to_string(),
            description: "".to_string(),
            notify_type: 0,
            time_to_live: 0,
            time_to_send: 0,
            notify_id: 0,
            registration_id: "".to_string(),
            alias: "".to_string(),
            user_account: "".to_string(),
            extra_sound_uri: "".to_string(),
            extra_ticker: "".to_string(),
            extra_notify_foreground: "".to_string(),
            extra_notify_effect: "".to_string(),
            extra_intent_uri: "".to_string(),
            extra_web_uri: "".to_string(),
            extra_flow_control: 0,
            extra_job_key: "".to_string(),
            extra_callback: "".to_string(),
            extra_callback_param: "".to_string(),
            extra_callback_type: 0,
            extra_locale: "".to_string(),
            extra_locale_not_in: "".to_string(),
            extra_model: "".to_string(),
            extra_model_not_in: "".to_string(),
            extra_app_version: "".to_string(),
            extra_app_version_not_in: "".to_string(),
            extra_network: "".to_string(),
            extra_only_send_once: "".to_string(),
        }
    }

    pub fn build(self) -> Message {
        Message {
            payload: self.payload,
            restricted_package_name: self.restricted_package_name,
            pass_through: self.pass_through,
            title: self.title,
            description: self.description,
            notify_type: self.notify_type,
            time_to_live: self.time_to_live,
            time_to_send: self.time_to_send,
            notify_id: self.notify_id,
            registration_id: self.registration_id,
            alias: self.alias,
            user_account: self.user_account,
            extra_sound_uri: self.extra_sound_uri,
            extra_ticker: self.extra_ticker,
            extra_notify_foreground: self.extra_notify_foreground,
            extra_notify_effect: self.extra_notify_effect,
            extra_intent_uri: self.extra_intent_uri,
            extra_web_uri: self.extra_web_uri,
            extra_flow_control: self.extra_flow_control,
            extra_job_key: self.extra_job_key,
            extra_callback: self.extra_callback,
            extra_callback_param: self.extra_callback_param,
            extra_callback_type: self.extra_callback_type,
            extra_locale: self.extra_locale,
            extra_locale_not_in: self.extra_locale_not_in,
            extra_model: self.extra_model,
            extra_model_not_in: self.extra_model_not_in,
            extra_app_version: self.extra_app_version,
            extra_app_version_not_in: self.extra_app_version_not_in,
            extra_network: self.extra_network,
            extra_only_send_once: self.extra_only_send_once,
        }
    }
}
