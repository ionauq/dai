use serde::{Deserialize, Serialize};
/// 华为开发平台配置信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HuaweiConfig {
    pub app_id: i32,
    pub client_id: String,
    pub client_secret: String,
}
