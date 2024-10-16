use std::error::Error;

pub trait Transmit {
    fn init_message_file(&self);
    fn send(&self) -> Result<(), Box<dyn Error>>;
}
