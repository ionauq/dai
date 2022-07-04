use serde::{Deserialize, Serialize};
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
            r#type: 1,
            intent: "".to_string(),
            url: "".to_string(),
            action: "".to_string(),
        }
    }
    pub fn r#type(mut self, r#type: i32) -> ClickActionBuilder {
        self.r#type = r#type;
        self
    }
    pub fn intent(mut self, intent: String) -> ClickActionBuilder {
        self.intent = intent;
        self
    }
    pub fn url(mut self, url: String) -> ClickActionBuilder {
        self.url = url;
        self
    }
    pub fn action(mut self, action: String) -> ClickActionBuilder {
        self.action = action;
        self
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
