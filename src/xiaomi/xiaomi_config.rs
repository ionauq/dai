use serde::{Deserialize, Serialize};
/// 小米开发平台配置信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XiaomiConfig {
    pub app_secret: String,
}
