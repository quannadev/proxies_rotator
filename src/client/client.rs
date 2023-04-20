use crate::client::auth::AuthProxy;
use crate::client::socket_address_type::SocksAddrType;
use crate::errors::ClientError;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug)]
struct Client {
    addr: SocketAddr,
    port: i16,
    auth: Option<AuthProxy>,
    proxy: TcpStream,
}

impl Client {
    pub async fn new(
        addr: SocketAddr,
        port: i16,
        auth: Option<AuthProxy>,
    ) -> Result<Self, ClientError> {
        let proxy = Self::init_client(addr, auth.clone()).await?;
        Ok(Self {
            port,
            auth,
            addr,
            proxy,
        })
    }

    pub async fn connect(&mut self, addr: SocksAddrType<'_>) -> Result<(), ClientError> {
        let mut buf = [0u8; 2];
        self.proxy
            .read_exact(&mut buf)
            .await
            .map_err(|e| ClientError::Connect(e.to_string()))?;

        let mut buf = addr.to_bytes();
        buf.extend(&self.port.to_be_bytes());
        self.proxy
            .write_all(&buf)
            .await
            .map_err(|e| ClientError::Connect(e.to_string()))?;
        Ok(())
    }

    async fn init_client(
        addr: SocketAddr,
        auth: Option<AuthProxy>,
    ) -> Result<TcpStream, ClientError> {
        // Send handshake to the proxy server
        log::debug!("start handshake");
        let mut proxy = TcpStream::connect(addr)
            .await
            .map_err(|e| ClientError::Connect(e.to_string()))?;

        proxy
            .write_all(&[5, 1, 0])
            .await
            .map_err(|e| ClientError::HandShake(e.to_string()))?;

        let mut buf = [0u8; 1024];
        let mut n = 0;

        n = proxy
            .read(&mut buf)
            .await
            .map_err(|e| ClientError::HandShake(e.to_string()))?;
        if n < 2 || buf[0] != 5 {
            return Err(ClientError::HandShake("Invalid proxy response".to_string()));
        }
        if let Some(auth) = &auth {
            // Send username/password authentication request to the proxy server
            log::debug!("send auth to proxy");
            let mut auth_buf = Vec::new();
            auth_buf.push(1); // Version of the sub-negotiation protocol
            auth_buf.push(auth.username.len() as u8); // Length of the username field
            auth_buf.extend_from_slice(auth.username.as_bytes()); // Username
            auth_buf.push(auth.password.len() as u8); // Length of the password field
            auth_buf.extend_from_slice(auth.password.as_bytes()); // Password
            proxy
                .write_all(&auth_buf)
                .await
                .map_err(|e| ClientError::Auth(e.to_string()))?
        }

        Ok(proxy)
    }

    pub async fn handshake_target<'a>(
        &self,
        socket: &mut TcpStream,
        addr_buf: &'a mut [u8],
    ) -> Result<(SocksAddrType<'a>, u16), ClientError> {
        let mut buf = [0u8; 2];
        socket
            .read_exact(&mut buf)
            .await
            .map_err(|e| ClientError::HandShake(e.to_string()))?;

        //validate socket version
        if buf[0] != 5 {
            return Err(ClientError::HandShake(
                "Unexpected socks version".to_string(),
            ));
        }
        let n = buf[1] as usize;

        if n == 0 {
            return Err(ClientError::HandShake(
                "Got empty list of supported authentication methods".to_string(),
            ));
        }
        let mut buf = [0u8; 255];
        socket
            .read_exact(&mut buf[..n])
            .await
            .map_err(|_| ClientError::HandShake("Failed to read handshake".to_string()))?;

        //send handshake
        socket
            .write_all(&[0x05, 0x00])
            .await
            .map_err(|_| ClientError::HandShake("Failed to send handshake".to_string()))?;

        let mut buf = [0u8; 4];
        socket
            .read_exact(&mut buf)
            .await
            .map_err(|_| ClientError::HandShake("Failed to read handshake".to_string()))?;

        if buf[0] != 5 {
            return Err(ClientError::HandShake(
                "Unexpected socks version".to_string(),
            ));
        }

        if buf[1] != 1 {
            return Err(ClientError::HandShake(
                "Only tcp/ip stream connections are supported".to_string(),
            ));
        }

        if buf[2] != 0 {
            return Err(ClientError::HandShake(
                "Reserved field is not zero".to_string(),
            ));
        }
        let addr =
            match buf[3] {
                1 => {
                    let buf = &mut addr_buf[..4];
                    socket.read_exact(buf).await.map_err(|_| {
                        ClientError::HandShake("Failed to read handshake".to_string())
                    })?;
                    SocksAddrType::Ipv4(buf)
                }
                3 => {
                    let n = socket.read_u8().await.map_err(|_| {
                        ClientError::HandShake("Failed to read handshake".to_string())
                    })? as usize;
                    let buf = &mut addr_buf[..n];
                    socket.read_exact(buf).await.map_err(|_| {
                        ClientError::HandShake("Failed to read handshake".to_string())
                    })?;
                    SocksAddrType::Domain(buf)
                }
                4 => {
                    let buf = &mut addr_buf[..16];
                    socket.read_exact(buf).await.map_err(|_| {
                        ClientError::HandShake("Failed to read handshake".to_string())
                    })?;
                    SocksAddrType::Ipv6(buf)
                }
                _x => {
                    return Err(ClientError::HandShake(
                        "Unsupported address type".to_string(),
                    ))
                }
            };
        let port = socket
            .read_u16()
            .await
            .map_err(|_| ClientError::HandShake("Failed to read port from socket".to_string()))?;

        Ok((addr, port))
    }
}
