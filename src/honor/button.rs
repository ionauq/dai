use serde::{Deserialize, Serialize};
/// 通知栏消息动作按钮
#[derive(Serialize, Deserialize, Debug)]
pub struct Button {
    name: String,
    #[serde(rename(serialize = "actionType", deserialize = "actionType"))]
    action_type: i32,
    #[serde(rename(serialize = "intentType", deserialize = "intentType"))]
    intent_type: i32,
    intent: String,
    data: String,
}

/// A builder to construct the properties of a `Button`.
pub struct ButtonBuilder {
    name: String,
    action_type: i32,
    intent_type: i32,
    intent: String,
    data: String,
}

impl ButtonBuilder {
    /// Creates a new `ButtonBuilder` with default properties.
    pub fn new() -> ButtonBuilder {
        ButtonBuilder {
            name: "".to_string(),
            action_type: 0,
            intent_type: 0,
            intent: "".to_string(),
            data: "".to_string(),
        }
    }
    /// Sets the name of the button.
    pub fn name(mut self, name: String) -> ButtonBuilder {
        self.name = name;
        self
    }
    /// Sets the action type of the button.
    pub fn action_type(mut self, action_type: i32) -> ButtonBuilder {
        self.action_type = action_type;
        self
    }
    /// Sets the intent type of the button.
    pub fn intent_type(mut self, intent_type: i32) -> ButtonBuilder {
        self.intent_type = intent_type;
        self
    }
    /// Sets the intent of the button.
    pub fn intent(mut self, intent: String) -> ButtonBuilder {
        self.intent = intent;
        self
    }
    /// Sets the data of the button.
    pub fn data(mut self, data: String) -> ButtonBuilder {
        self.data = data;
        self
    }
    /// Builds a `Button` with the properties set on this builder.
    pub fn build(self) -> Button {
        Button {
            name: self.name,
            action_type: self.action_type,
            intent_type: self.intent_type,
            intent: self.intent,
            data: self.data,
        }
    }
}
