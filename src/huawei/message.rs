#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestMessage {
    pub validate_only: bool,
    pub message: Message,
}

pub struct RequestMessageBuilder {
    validate_only: bool,
    message: Message,
}

impl RequestMessageBuilder {
    pub fn new() -> RequestMessageBuilder {
        RequestMessageBuilder {
            validate_only: false,
            message: MessageBuilder::new().build(),
        }
    }

    pub fn build(self) -> RequestMessage {
        RequestMessage {
            validate_only: self.validate_only,
            message: self.message,
        }
    }
}

/// 推送消息结构体，message结构体中必须存在有效消息负载以及有效发送目标
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub data: String,
    pub notification: Notification,
    pub android: AndroidConfig,
    pub token: Vec<String>,
    pub topic: String,
    pub condition: String,
}

pub struct MessageBuilder {
    data: String,
    notification: Notification,
    android: AndroidConfig,
    token: Vec<String>,
    topic: String,
    condition: String,
}

impl MessageBuilder {
    pub fn new() -> MessageBuilder {
        MessageBuilder {
            data: "".to_string(),
            notification: NotificationBuilder::new().build(),
            android: AndroidConfigBuilder::new().build(),
            token: vec![],
            topic: "".to_string(),
            condition: "".to_string(),
        }
    }

    pub fn build(self) -> Message {
        Message {
            data: self.data,
            notification: self.notification,
            android: self.android,
            token: self.token,
            topic: self.topic,
            condition: self.condition,
        }
    }
}

/// 通知栏消息内容
#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub title: String,
    pub body: String,
    pub image: String,
}

pub struct NotificationBuilder {
    title: String,
    body: String,
    image: String,
}

impl NotificationBuilder {
    pub fn new() -> NotificationBuilder {
        NotificationBuilder {
            title: "".to_string(),
            body: "".to_string(),
            image: "".to_string(),
        }
    }

    pub fn build(self) -> Notification {
        Notification {
            title: self.title,
            body: self.body,
            image: self.image,
        }
    }
}

/// Android消息推送控制参数
#[derive(Serialize, Deserialize, Debug)]
pub struct AndroidConfig {
    pub collapse_key: i64,
    pub urgency: String,
    pub category: String,
    pub ttl: String,
    pub bi_tag: String,
    pub receipt_id: String,
    pub fast_app_target: i64,
    pub data: String,
    pub notification: AndroidNotification,
}

pub struct AndroidConfigBuilder {
    collapse_key: i64,
    urgency: String,
    category: String,
    ttl: String,
    bi_tag: String,
    receipt_id: String,
    fast_app_target: i64,
    data: String,
    notification: AndroidNotification,
}

impl AndroidConfigBuilder {
    pub fn new() -> AndroidConfigBuilder {
        AndroidConfigBuilder {
            collapse_key: 0,
            urgency: "".to_string(),
            category: "".to_string(),
            ttl: "".to_string(),
            bi_tag: "".to_string(),
            receipt_id: "".to_string(),
            fast_app_target: 0,
            data: "".to_string(),
            notification: AndroidNotificationBuilder::new().build(),
        }
    }

    pub fn build(self) -> AndroidConfig {
        AndroidConfig {
            collapse_key: self.collapse_key,
            urgency: self.urgency,
            category: self.category,
            ttl: self.ttl,
            bi_tag: self.bi_tag,
            receipt_id: self.receipt_id,
            fast_app_target: self.fast_app_target,
            data: self.data,
            notification: self.notification,
        }
    }
}

/// Android通知栏消息结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct AndroidNotification {
    pub title: String,
    pub body: String,
    pub icon: String,
    pub color: String,
    pub sound: String,
    pub default_sound: bool,
    pub tag: String,
    pub click_action: ClickAction,
    pub body_loc_key: String,
    pub body_loc_args: Vec<String>,
    pub title_loc_key: String,
    pub title_loc_args: Vec<String>,
    pub channel_id: String,
    pub notify_summary: String,
    pub image: String,
    pub style: i64,
    pub big_title: String,
    pub big_body: String,
    pub notify_id: i64,
    pub group: String,
    pub badge: BadgeNotification,
    pub ticker: String,
    pub when: String,
    pub importance: String,
    pub use_default_vibrate: bool,
    pub use_default_light: bool,
    pub local_only: bool,
    pub visibility: String,
    pub vibrate_config: Vec<String>,
    pub light_settings: LightSettings,
    pub foreground_show: bool,
    pub profile_id: String,
    pub inbox_content: Vec<String>,
    pub buttons: Vec<Button>,
}

struct AndroidNotificationBuilder {
    title: String,
    body: String,
    icon: String,
    color: String,
    sound: String,
    default_sound: bool,
    tag: String,
    click_action: ClickAction,
    body_loc_key: String,
    body_loc_args: Vec<String>,
    title_loc_key: String,
    title_loc_args: Vec<String>,
    channel_id: String,
    notify_summary: String,
    image: String,
    style: i64,
    big_title: String,
    big_body: String,
    notify_id: i64,
    group: String,
    badge: BadgeNotification,
    ticker: String,
    when: String,
    importance: String,
    use_default_vibrate: bool,
    use_default_light: bool,
    local_only: bool,
    visibility: String,
    vibrate_config: Vec<String>,
    light_settings: LightSettings,
    foreground_show: bool,
    profile_id: String,
    inbox_content: Vec<String>,
    buttons: Vec<Button>,
}

impl AndroidNotificationBuilder {
    pub fn new() -> AndroidNotificationBuilder {
        AndroidNotificationBuilder {
            title: "".to_string(),
            body: "".to_string(),
            icon: "".to_string(),
            color: "".to_string(),
            sound: "".to_string(),
            default_sound: false,
            tag: "".to_string(),
            click_action: ClickActionBuilder::new().build(),
            body_loc_key: "".to_string(),
            body_loc_args: vec![],
            title_loc_key: "".to_string(),
            title_loc_args: vec![],
            channel_id: "".to_string(),
            notify_summary: "".to_string(),
            image: "".to_string(),
            style: 0,
            big_title: "".to_string(),
            big_body: "".to_string(),
            notify_id: 0,
            group: "".to_string(),
            badge: BadgeNotificationBuilder::new().build(),
            ticker: "".to_string(),
            when: "".to_string(),
            importance: "".to_string(),
            use_default_vibrate: false,
            use_default_light: false,
            local_only: false,
            visibility: "".to_string(),
            vibrate_config: vec![],
            light_settings: LightSettingsBuilder::new().build(),
            foreground_show: false,
            profile_id: "".to_string(),
            inbox_content: vec![],
            buttons: vec![],
        }
    }

    pub fn build(self) -> AndroidNotification {
        AndroidNotification {
            title: self.title,
            body: self.body,
            icon: self.icon,
            color: self.color,
            sound: self.sound,
            default_sound: self.default_sound,
            tag: self.tag,
            click_action: self.click_action,
            body_loc_key: self.body_loc_key,
            body_loc_args: self.body_loc_args,
            title_loc_key: self.title_loc_key,
            title_loc_args: self.title_loc_args,
            channel_id: self.channel_id,
            notify_summary: self.notify_summary,
            image: self.image,
            style: self.style,
            big_title: self.big_title,
            big_body: self.big_body,
            notify_id: self.notify_id,
            group: self.group,
            badge: self.badge,
            ticker: self.ticker,
            when: self.when,
            importance: self.importance,
            use_default_vibrate: self.use_default_vibrate,
            use_default_light: self.use_default_light,
            local_only: self.local_only,
            visibility: self.visibility,
            vibrate_config: self.vibrate_config,
            light_settings: self.light_settings,
            foreground_show: self.foreground_show,
            profile_id: self.profile_id,
            inbox_content: self.inbox_content,
            buttons: self.buttons,
        }
    }
}

/// Android通知消息角标控制
#[derive(Serialize, Deserialize, Debug)]
pub struct BadgeNotification {
    pub add_num: i32,
    pub class: String,
    pub set_num: i32,
}

pub struct BadgeNotificationBuilder {
    add_num: i32,
    class: String,
    set_num: i32,
}

impl BadgeNotificationBuilder {
    pub fn new() -> BadgeNotificationBuilder {
        BadgeNotificationBuilder {
            add_num: 1,
            class: "".to_string(),
            set_num: 0,
        }
    }

    pub fn build(self) -> BadgeNotification {
        BadgeNotification {
            add_num: self.add_num,
            class: self.class,
            set_num: self.set_num,
        }
    }
}

/// 消息点击行为
#[derive(Serialize, Deserialize, Debug)]
pub struct ClickAction {
    pub r#type: i32,
    pub intent: String,
    pub url: String,
    pub action: String,
}

pub struct ClickActionBuilder {
    r#type: i32,
    intent: String,
    url: String,
    action: String,
}

impl ClickActionBuilder {
    pub fn new() -> ClickActionBuilder {
        ClickActionBuilder {
            r#type: 0,
            intent: "".to_string(),
            url: "".to_string(),
            action: "".to_string(),
        }
    }

    pub fn build(self) -> ClickAction {
        ClickAction {
            r#type: self.r#type,
            intent: self.intent,
            url: self.url,
            action: self.action,
        }
    }
}

/// 通知栏消息动作按钮
#[derive(Serialize, Deserialize, Debug)]
pub struct Button {
    pub name: String,
    pub action_type: i32,
    pub intent_type: i32,
    pub intent: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightSettings {
    pub color: Color,
    pub light_on_duration: String,
    pub light_off_duration: String,
}

pub struct LightSettingsBuilder {
    color: Color,
    light_on_duration: String,
    light_off_duration: String,
}

impl LightSettingsBuilder {
    pub fn new() -> LightSettingsBuilder {
        LightSettingsBuilder {
            color: ColorBuilder::new().build(),
            light_on_duration: "".to_string(),
            light_off_duration: "".to_string(),
        }
    }

    pub fn build(self) -> LightSettings {
        LightSettings {
            color: self.color,
            light_on_duration: self.light_on_duration,
            light_off_duration: self.light_off_duration,
        }
    }
}

/// 呼吸灯颜色
#[derive(Serialize, Deserialize, Debug)]
pub struct Color {
    pub alpha: i64,
    pub red: i64,
    pub green: i64,
    pub blue: i64,
}

pub struct ColorBuilder {
    alpha: i64,
    red: i64,
    green: i64,
    blue: i64,
}

impl ColorBuilder {
    pub fn new() -> ColorBuilder {
        ColorBuilder {
            alpha: 0,
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    pub fn build(self) -> Color {
        Color {
            alpha: self.alpha,
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }
}
