use std::io;

use async_trait::async_trait;
use rustls::ServerConfig;
use tokio::net::TcpListener;

use crate::proxy::TcpInboundHandler;

use super::Listener;

struct TlsListener {
    h: TcpInboundHandler,
    listener: Option<TcpListener>,
}

impl TlsListener {
    pub fn new(h: TcpInboundHandler, listener: TcpListener) -> Self {
        Self {
            h,
            listener: Some(listener),
        }
    }
}
#[async_trait]
impl Listener for TlsListener {
    async fn listen(&mut self) -> io::Result<()> {
        let h = self.h.clone();
        let listener = match self.listener.take() {
            Some(l) => l,
            None => {
                panic!("listen only allow call once");
            }
        };
        async move {
            loop {
                let (stream, addr) = match listener.accept().await {
                    Ok(x) => x,
                    Err(err) => {
                        log::error!("{:?}", err);
                        continue;
                    }
                };
                let h = h.clone();
                tokio::task::spawn_local(async move {
                    if let Err(err) = h.process_incoming(stream).await {
                        log::error!("{:?}", err);
                    }
                    Ok::<_, io::Error>(())
                });
            }
        }
        .await;
        Ok(())
    }
}
