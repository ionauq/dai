use crate::common::request::RequestToken;
use crate::common::secret::Secret;
use crate::honor::HONOR_CREDENTIAL_URL;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

/// 荣耀开发平台配置信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HonorConfig {
    pub app_id: i32,
    pub client_id: String,
    pub client_secret: String,
}

impl Secret for HonorConfig {}
