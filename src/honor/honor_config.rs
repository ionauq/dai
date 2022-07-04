use serde::{Deserialize, Serialize};
/// 荣耀开发平台配置信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HonorConfig {
    pub app_id: i32,
    pub client_id: String,
    pub client_secret: String,
}
