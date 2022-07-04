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
    pub icon: String,
    #[serde(rename(serialize = "clickAction", deserialize = "clickAction"))]
    pub click_action: ClickAction,
    #[serde(rename(serialize = "notifySummary", deserialize = "notifySummary"))]
    pub notify_summary: String,
    pub image: String,
    pub style: i32,
    #[serde(rename(serialize = "bigTitle", deserialize = "bigTitle"))]
    pub big_title: String,
    #[serde(rename(serialize = "bigBody", deserialize = "bigBody"))]
    pub big_body: String,
    pub when: String,
    pub buttons: Vec<Button>,
    pub badge: BadgeNotification,
}

pub struct AndroidNotificationBuilder {
    title: String,
    body: String,
    icon: String,
    click_action: ClickAction,
    notify_summary: String,
    image: String,
    style: i32,
    big_title: String,
    big_body: String,
    when: String,
    buttons: Vec<Button>,
    badge: BadgeNotification,
}

impl AndroidNotificationBuilder {
    pub fn new() -> AndroidNotificationBuilder {
        AndroidNotificationBuilder {
            title: "".to_string(),
            body: "".to_string(),
            icon: "".to_string(),
            click_action: ClickActionBuilder::new().build(),
            notify_summary: "".to_string(),
            image: "".to_string(),
            style: 0,
            big_title: "".to_string(),
            big_body: "".to_string(),
            when: "".to_string(),
            buttons: vec![],
            badge: BadgeNotificationBuilder::new().build(),
        }
    }
    pub fn title(mut self, title: String) -> AndroidNotificationBuilder {
        self.title = title;
        self
    }
    pub fn body(mut self, body: String) -> AndroidNotificationBuilder {
        self.body = body;
        self
    }
    pub fn icon(mut self, icon: String) -> AndroidNotificationBuilder {
        self.icon = icon;
        self
    }
    pub fn click_action(mut self, click_action: ClickAction) -> AndroidNotificationBuilder {
        self.click_action = click_action;
        self
    }
    pub fn notify_summary(mut self, notify_summary: String) -> AndroidNotificationBuilder {
        self.notify_summary = notify_summary;
        self
    }
    pub fn image(mut self, image: String) -> AndroidNotificationBuilder {
        self.image = image;
        self
    }
    pub fn style(mut self, style: i32) -> AndroidNotificationBuilder {
        self.style = style;
        self
    }
    pub fn big_title(mut self, big_title: String) -> AndroidNotificationBuilder {
        self.big_title = big_title;
        self
    }
    pub fn big_body(mut self, big_body: String) -> AndroidNotificationBuilder {
        self.big_body = big_body;
        self
    }
    pub fn when(mut self, when: String) -> AndroidNotificationBuilder {
        self.when = when;
        self
    }

    pub fn buttons(mut self, buttons: Vec<Button>) -> AndroidNotificationBuilder {
        self.buttons = buttons;
        self
    }

    pub fn badge(mut self, badge: BadgeNotification) -> AndroidNotificationBuilder {
        self.badge = badge;
        self
    }

    pub fn build(self) -> AndroidNotification {
        AndroidNotification {
            title: self.title,
            body: self.body,
            icon: self.icon,
            click_action: self.click_action,
            notify_summary: self.notify_summary,
            image: self.image,
            style: self.style,
            big_title: self.big_title,
            big_body: self.big_body,
            when: self.when,
            buttons: self.buttons,
            badge: self.badge,
        }
    }
}
