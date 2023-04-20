use crate::ProxyAuth;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Proxy {
    pub addr: SocketAddr,
    pub port: u16,
    pub auth: ProxyAuth,
}
