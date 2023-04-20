#![allow(clippy::needless_lifetimes)]

pub mod errors;
pub mod list;

use clap::Parser;
use env_logger::Env;
use proxies_rotator::{Config, ProxyServer};

#[tokio::main]
async fn main() {
    let cnf = Config::parse();
    env_logger::init_from_env(Env::default().default_filter_or(cnf.get_logging_cnf()));

    let server = ProxyServer::new(&cnf).await;
    server.run().await
}
