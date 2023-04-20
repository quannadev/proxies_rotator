use crate::ProxyAuth;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Proxy {
    pub addr: SocketAddr,
    pub auth: ProxyAuth,
}

impl TryFrom<String> for Proxy {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('|').collect();
        if parts.is_empty() {
            return Err(());
        }
        if parts.len() < 2 {
            return Err(());
        }
        let addr = parts[0]
            .parse::<SocketAddr>()
            .expect("invalid socket address");

        let credentials = parts[1];
        let creds: Vec<&str> = credentials.split(':').collect();
        let username = creds[0].to_string();
        let password = creds[1].to_string();
        let auth = ProxyAuth { username, password };
        Ok(Self { addr, auth })
    }
}
