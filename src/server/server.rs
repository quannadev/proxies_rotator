use crate::client::Client;
use crate::server::proxy::Proxy;
use crate::Config;
use arc_swap::ArcSwap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct ProxyServer {
    listener: TcpListener,
    proxies: Vec<Proxy>,
}

impl ProxyServer {
    pub async fn new(cnf: &Config) -> Self {
        let listener = TcpListener::bind(cnf.bind)
            .await
            .expect("invalid server address");
        let proxies = cnf
            .load_proxies_from_path()
            .await
            .expect("load proxies error");
        Self {
            listener,
            proxies: Vec::new(),
        }
    }

    pub async fn run(&self) {
        let proxies = ArcSwap::new(Arc::new(self.proxies.clone()));
        loop {
            tokio::select! {
                res = self.listener.accept() => {
                    let (mut socket, src) = match res {
                        Ok(x) => x,
                        Err(err) => {
                            log::error!("Failed to accept connection: {:#}", err);
                            continue;
                        },
                    };
                    log::debug!("Got new client connection from {}", src);
                    let proxies = proxies.load();

                    tokio::spawn(async move {
                        //todo rotate proxy
                        let proxy = proxies.first().unwrap().clone();
                        match Client::new(proxy.addr, proxy.port, Some(proxy.auth)).await {
                            Ok(mut client) => {
                                //todo serve to socket
                                 let mut buf = [0u8; 255];
                                 match client.handshake_target(&mut socket, &mut buf).await {
                                    Ok((addr, port)) => {
                                       let _ = client.connect(addr, port).await;
                                        match tokio::io::copy_bidirectional(&mut socket, &mut client.proxy).await {
                                            Ok(value) => {},
                                            Err(err) => {}
                                        }

                                    },
                                    Err(err) => {
                                        log::error!("connect proxy error: {:?}", err);
                                    }
                                }
                            },
                            Err(err) => {
                                log::error!("connect proxy error: {:?}", err);
                            }
                        }
                    });
                }
            }
        }
    }
}
