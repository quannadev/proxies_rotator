use crate::errors::ServerError;
use crate::Proxy;
use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, BufReader};

#[derive(Debug, Clone, Parser)]
pub struct Config {
    #[clap(short, long, env, default_value_t = false)]
    pub quiet: bool,
    #[clap(short, long, env, default_value_t = 0)]
    pub verbose: u8,
    #[clap(short, long, env, default_value = "0.0.0.0:1379")]
    pub bind: SocketAddr,
    #[clap(short, long, env)]
    pub proxy_list: PathBuf,
}

impl Config {
    pub fn get_logging_cnf(&self) -> &str {
        match (self.quiet, self.verbose) {
            (true, _) => "warn",
            (false, 0) => "info",
            (false, 1) => "info,proxies_rotator=debug",
            (false, 2) => "debug",
            (false, _) => "debug,proxies_rotator=trace",
        }
    }

    pub async fn load_proxies_from_path(&self) -> Result<Vec<Proxy>, ServerError> {
        let f = File::open(self.proxy_list.clone()).await.map_err(|_| {
            ServerError::Parser(format!("Failed to open file: {:?}", self.proxy_list))
        })?;
        let f = BufReader::new(f);
        Self::load_proxies_from_reader(f).await
    }

    pub async fn load_proxies_from_reader<T: AsyncBufRead + Unpin>(
        f: T,
    ) -> Result<Vec<Proxy>, ServerError> {
        let mut proxies = Vec::new();

        let mut lines = f.lines();
        while let Some(line) = lines
            .next_line()
            .await
            .map_err(|_| ServerError::Parser("read proxy error".to_string()))?
        {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Ok(proxy) = Proxy::try_from(line.clone()) {
                proxies.push(proxy);
            }
        }

        log::info!("Loaded {} proxies from file", proxies.len());

        Ok(proxies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use tokio::io::BufReader;

    #[tokio::test]
    async fn test_parse_ipv4() {
        let buf = b"192.0.2.1:1337";
        let reader = BufReader::new(Cursor::new(buf));
        let list = Config::load_proxies_from_reader(reader).await.unwrap();
        for proxy in list {
            assert!(proxy.auth.is_none());
            assert!(proxy.addr.is_ipv4());
            assert_eq!(proxy.addr.port(), 1337);
            assert_eq!(proxy.addr.to_string(), "192.0.2.1:1337")
        }
    }
    #[tokio::test]
    async fn test_parse_ipv6() {
        env_logger::init();
        let buf = b"[2001:db8::12:34]:1337";
        let reader = BufReader::new(Cursor::new(buf));
        let list = Config::load_proxies_from_reader(reader).await.unwrap();
        for proxy in list {
            assert!(proxy.auth.is_none());
            assert!(proxy.addr.is_ipv6());
            assert_eq!(proxy.addr.port(), 1337);
            assert_eq!(proxy.addr.to_string(), "[2001:db8::12:34]:1337")
        }
    }
}
