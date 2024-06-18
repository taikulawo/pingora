use async_trait::async_trait;
use std::{io, sync::Arc};

use tokio::{io::{AsyncRead, AsyncWrite}, net::TcpStream};
mod http;
mod tls;
mod pingora;
#[async_trait]
pub trait TcpInbound {
    async fn process_incoming(&self, stream: TcpStream) -> io::Result<()>;
}
pub trait TcpOutbound {}
pub type TcpInboundHandler = Arc<dyn TcpInbound + Send + Sync + Unpin>;
pub type TcpOutboundHandler = Arc<dyn TcpOutbound + Send + Sync + Unpin>;

trait Stream: AsyncRead + AsyncWrite + Send + Unpin + 'static {}

impl<T> Stream for T where T: AsyncRead + AsyncWrite + Send + Unpin + 'static {}

type AnyStream = Box<dyn Stream>;
