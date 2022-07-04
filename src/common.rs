use std::fmt;

/// 通道枚举类
pub enum ChannelType {
    APNs,
    Huawei,
    Xiaomi,
    Oppo,
    Vivo,
    Honor,
    Meizu,
    FCM,
}

impl fmt::Display for ChannelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChannelType::APNs => write!(f, "apns"),
            ChannelType::Huawei => write!(f, "huawei"),
            ChannelType::Xiaomi => write!(f, "xiaomi"),
            ChannelType::Oppo => write!(f, "oppo"),
            ChannelType::Vivo => write!(f, "vivo"),
            ChannelType::Honor => write!(f, "honor"),
            ChannelType::Meizu => write!(f, "meizu"),
            ChannelType::FCM => write!(f, "fcm"),
        }
    }
}
