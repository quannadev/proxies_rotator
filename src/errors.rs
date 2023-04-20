pub use log::{debug, error, info, trace, warn};

#[derive(Clone, Debug)]
pub enum ClientError {
    Connect(String),
    Auth(String),
    HandShake(String),
}

#[derive(Clone, Debug)]
pub enum ServerError {
    Parser(String),
}
