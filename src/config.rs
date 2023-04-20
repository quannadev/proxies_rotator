use std::net::SocketAddr;
use std::path::PathBuf;
use clap;
use clap::Parser;

#[derive(Debug, Default, Clone, Parser)]
pub struct Config {
    #[clap(short="q", long, env)]
    pub quiet: bool,
    #[clap(short="v", long, env)]
    pub verbose: u8,
    #[clap(short="b", long, env)]
    pub bind: SocketAddr,
    #[clap(short="l",long, env)]
    pub proxy_list: PathBuf,
}