pub use log::{debug, error, info, trace, warn};
use std::io::Error;

#[derive(Clone, Debug)]
pub enum ClientError {
    Connect(String),
    Auth(String),
    HandShake(String),
}
