use crate::ProxyAuth;
use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Proxy {
    pub addr: SocketAddr,
    pub auth: Option<ProxyAuth>,
}

impl TryFrom<String> for Proxy {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('|').collect();
        if parts.is_empty() {
            return Err(());
        }
        if parts.len() < 1 {
            return Err(());
        }
        let addr = parts[0]
            .parse::<SocketAddr>()
            .expect("invalid socket address");

        if parts.len() > 1 {
            let credentials = parts[1];
            let creds: Vec<&str> = credentials.split(':').collect();
            let username = creds[0].to_string();
            let password = creds[1].to_string();
            let auth = ProxyAuth { username, password };
            return Ok(Self {
                addr,
                auth: Some(auth),
            });
        }

        Ok(Self { addr, auth: None })
    }
}
