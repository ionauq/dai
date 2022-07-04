use serde::{Deserialize, Serialize};

/// Android通知消息角标控制
#[derive(Serialize, Deserialize, Debug)]
pub struct BadgeNotification {
    #[serde(rename(serialize = "addNum", deserialize = "addNum"))]
    pub add_num: i32,
    #[serde(rename(serialize = "badgeClass", deserialize = "badgeClass"))]
    pub badge_class: String,
    #[serde(rename(serialize = "setNum", deserialize = "setNum"))]
    pub set_num: i32,
}

pub struct BadgeNotificationBuilder {
    add_num: i32,
    badge_class: String,
    set_num: i32,
}

impl BadgeNotificationBuilder {
    pub fn new() -> BadgeNotificationBuilder {
        BadgeNotificationBuilder {
            add_num: 1,
            badge_class: "".to_string(),
            set_num: 0,
        }
    }
    pub fn add_num(mut self, add_num: i32) -> BadgeNotificationBuilder {
        self.add_num = add_num;
        self
    }
    pub fn badge_class(mut self, badge_class: String) -> BadgeNotificationBuilder {
        self.badge_class = badge_class;
        self
    }
    pub fn set_num(mut self, set_num: i32) -> BadgeNotificationBuilder {
        self.set_num = set_num;
        self
    }
    pub fn build(self) -> BadgeNotification {
        BadgeNotification {
            add_num: self.add_num,
            badge_class: self.badge_class,
            set_num: self.set_num,
        }
    }
}
