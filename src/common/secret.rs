use crate::common::request::RequestToken;
use std::future::Future;

pub trait Secret {
    // fn auth(&self) -> Result<RequestToken, Box<dyn std::error::Error>>;
    fn auth(&self);
}
