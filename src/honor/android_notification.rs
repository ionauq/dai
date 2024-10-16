use super::badge_notification::BadgeNotificationBuilder;
use super::button::Button;
use super::click_action::ClickAction;
use super::{badge_notification::BadgeNotification, click_action::ClickActionBuilder};
use serde::{Deserialize, Serialize};

/// Android通知栏消息结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct AndroidNotification {
    pub title: String,
    pub body: String,
    #[serde(rename(serialize = "clickAction", deserialize = "clickAction"))]
    pub click_action: ClickAction,
    pub image: String,
    pub style: i32,
    #[serde(rename(serialize = "bigTitle", deserialize = "bigTitle"))]
    pub big_title: String,
    #[serde(rename(serialize = "bigBody", deserialize = "bigBody"))]
    pub big_body: String,
    pub importance: String,
    pub when: String,
    pub buttons: Vec<Button>,
    pub badge: BadgeNotification,
    #[serde(rename(serialize = "notifyId", deserialize = "notifyId"))]
    pub notify_id: String,
}

pub struct AndroidNotificationBuilder {
    title: String,
    body: String,
    click_action: ClickAction,
    image: String,
    style: i32,
    big_title: String,
    big_body: String,
    importance: String,
    when: String,
    buttons: Vec<Button>,
    badge: BadgeNotification,
    notify_id: String,
}

impl AndroidNotificationBuilder {
    pub fn new() -> AndroidNotificationBuilder {
        AndroidNotificationBuilder {
            title: "".to_string(),
            body: "".to_string(),
            click_action: ClickActionBuilder::new().build(),
            image: "".to_string(),
            style: 0,
            big_title: "".to_string(),
            big_body: "".to_string(),
            importance: "".to_string(),
            when: "".to_string(),
            buttons: vec![],
            badge: BadgeNotificationBuilder::new().build(),
            notify_id: "".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn title(mut self, title: String) -> AndroidNotificationBuilder {
        self.title = title;
        self
    }

    #[allow(dead_code)]
    pub fn body(mut self, body: String) -> AndroidNotificationBuilder {
        self.body = body;
        self
    }

    #[allow(dead_code)]
    pub fn click_action(mut self, click_action: ClickAction) -> AndroidNotificationBuilder {
        self.click_action = click_action;
        self
    }

    #[allow(dead_code)]
    pub fn image(mut self, image: String) -> AndroidNotificationBuilder {
        self.image = image;
        self
    }

    #[allow(dead_code)]
    pub fn style(mut self, style: i32) -> AndroidNotificationBuilder {
        self.style = style;
        self
    }

    #[allow(dead_code)]
    pub fn big_title(mut self, big_title: String) -> AndroidNotificationBuilder {
        self.big_title = big_title;
        self
    }

    #[allow(dead_code)]
    pub fn big_body(mut self, big_body: String) -> AndroidNotificationBuilder {
        self.big_body = big_body;
        self
    }

    #[allow(dead_code)]
    pub fn importance(mut self, importance: String) -> AndroidNotificationBuilder {
        self.importance = importance;
        self
    }

    #[allow(dead_code)]
    pub fn when(mut self, when: String) -> AndroidNotificationBuilder {
        self.when = when;
        self
    }

    #[allow(dead_code)]
    pub fn buttons(mut self, buttons: Vec<Button>) -> AndroidNotificationBuilder {
        self.buttons = buttons;
        self
    }

    #[allow(dead_code)]
    pub fn badge(mut self, badge: BadgeNotification) -> AndroidNotificationBuilder {
        self.badge = badge;
        self
    }

    #[allow(dead_code)]
    pub fn notify_id(mut self, notify_id: String) -> AndroidNotificationBuilder {
        self.notify_id = notify_id;
        self
    }

    pub fn build(self) -> AndroidNotification {
        AndroidNotification {
            title: self.title,
            body: self.body,
            click_action: self.click_action,
            image: self.image,
            style: self.style,
            big_title: self.big_title,
            big_body: self.big_body,
            importance: self.importance,
            when: self.when,
            buttons: self.buttons,
            badge: self.badge,
            notify_id: self.notify_id,
        }
    }
}
