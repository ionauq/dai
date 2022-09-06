use std::fmt;

pub mod request;
pub mod request_token;

/// 通道枚举类
pub enum ChannelType {
    Apns,
    Huawei,
    Xiaomi,
    Oppo,
    Vivo,
    Honor,
    Meizu,
    Fcm,
}

impl fmt::Display for ChannelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChannelType::Apns => write!(f, "apns"),
            ChannelType::Huawei => write!(f, "huawei"),
            ChannelType::Xiaomi => write!(f, "xiaomi"),
            ChannelType::Oppo => write!(f, "oppo"),
            ChannelType::Vivo => write!(f, "vivo"),
            ChannelType::Honor => write!(f, "honor"),
            ChannelType::Meizu => write!(f, "meizu"),
            ChannelType::Fcm => write!(f, "fcm"),
        }
    }
}
