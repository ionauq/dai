use serde::{Deserialize, Serialize};

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
    #[allow(dead_code)]
    pub fn title(mut self, title: String) -> NotificationBuilder {
        self.title = title;
        self
    }
    #[allow(dead_code)]
    pub fn body(mut self, body: String) -> NotificationBuilder {
        self.body = body;
        self
    }
    #[allow(dead_code)]
    pub fn image(mut self, image: String) -> NotificationBuilder {
        self.image = image;
        self
    }

    pub fn build(self) -> Notification {
        Notification {
            title: self.title,
            body: self.body,
            image: self.image,
        }
    }
}
