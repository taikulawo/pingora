use async_trait::async_trait;
use tokio::net::TcpStream;

use crate::proxy::{AnyStream, TcpInbound};

pub struct TlsIncoming {}

#[async_trait]
impl TcpInbound for TlsIncoming {
    async fn process_incoming(&self, stream: TcpStream) -> std::io::Result<()> {
        todo!()
    }
}
