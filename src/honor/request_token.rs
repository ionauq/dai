use serde::{Deserialize, Serialize};

/// 鉴权信息
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestToken {
    pub access_token: String,
    pub expires_in: u32,
    pub token_type: String,
}
