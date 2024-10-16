use crate::ChannelType::{Apns, Fcm, Honor, Huawei, Meizu, Oppo, Vivo, Xiaomi};
use std::fmt;
use std::str::FromStr;

pub mod config;
pub mod request;
pub mod secret;
pub mod transmit;

/// 通道枚举类
#[derive(Debug, PartialEq, Eq)]
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
            Apns => write!(f, "apns"),
            Huawei => write!(f, "huawei"),
            Xiaomi => write!(f, "xiaomi"),
            Oppo => write!(f, "oppo"),
            Vivo => write!(f, "vivo"),
            Honor => write!(f, "honor"),
            Meizu => write!(f, "meizu"),
            Fcm => write!(f, "fcm"),
        }
    }
}
// 实现FromStr 支持字符串转枚举
impl FromStr for ChannelType {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "apns" => Ok(Apns),
            "huawei" => Ok(Huawei),
            "honor" => Ok(Honor),
            "xiaomi" => Ok(Xiaomi),
            "oppo" => Ok(Oppo),
            "vivo" => Ok(Vivo),
            "meizu" => Ok(Meizu),
            "fcm" => Ok(Fcm),
            _ => Err(()),
        }
    }
}
