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
}