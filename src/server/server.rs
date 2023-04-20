use crate::client::Client;
use crate::server::proxy::Proxy;
use crate::Config;
use arc_swap::ArcSwap;
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
        Self { listener, proxies }
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
                        let client = Client::default();
                        if let Ok(mut stream) = client.init_socket(proxy.addr, proxy.auth).await {
                            let mut buf = [0u8; 255];
                             match client.handshake_target(&mut socket, &mut buf).await {
                                Ok((addr, port)) => {
                                   let _ = client.connect(&mut stream, addr, port).await;
                                    if (tokio::io::copy_bidirectional(&mut socket, &mut stream).await).is_ok() {}
                                },
                                Err(err) => {
                                    log::error!("connect proxy error: {:?}", err);
                                }
                            }
                        }
                    });
                }
            }
        }
    }
}
