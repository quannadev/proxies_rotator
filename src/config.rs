use crate::errors::ServerError;
use clap;
use clap::Parser;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, BufReader};

#[derive(Debug, Clone, Parser)]
pub struct Config {
    #[clap(long, env)]
    pub quiet: bool,
    #[clap(long, env)]
    pub verbose: u8,
    #[clap(long, env)]
    pub bind: SocketAddr,
    #[clap(long, env)]
    pub proxy_list: PathBuf,
}

impl Config {
    pub fn get_logging_cnf(&self) -> &str {
        match (self.quiet, self.verbose) {
            (true, _) => "warn",
            (false, 0) => "info",
            (false, 1) => "info,laundry5=debug",
            (false, 2) => "debug",
            (false, _) => "debug,laundry5=trace",
        }
    }
    pub async fn load_proxies_from_path(&self) -> Result<Vec<SocketAddr>, ServerError> {
        let f = File::open(self.proxy_list.clone()).await.map_err(|_| {
            ServerError::Parser(format!("Failed to open file: {:?}", self.proxy_list))
        })?;
        let f = BufReader::new(f);
        Self::load_proxies_from_reader(f).await
    }

    async fn load_proxies_from_reader<T: AsyncBufRead + Unpin>(
        f: T,
    ) -> Result<Vec<SocketAddr>, ServerError> {
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

            let proxy = line
                .parse::<SocketAddr>()
                .map_err(|_| ServerError::Parser(format!("Invalid proxy in list: {:?}", line)))?;

            proxies.push(proxy);
        }

        log::info!("Loaded {} proxies from file", proxies.len());

        Ok(proxies)
    }
}
