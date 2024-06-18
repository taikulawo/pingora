use std::{io, net::SocketAddr, sync::Arc};

use async_trait::async_trait;
use socket2::{Domain, Socket, Type};
use tokio::net::TcpListener;

use crate::proxy::TcpInboundHandler;
mod tls;
#[async_trait]
trait Listener {
    async fn listen(&mut self) -> io::Result<()>;
}
type ListenerHandler = Arc<dyn Listener>;

struct ListenerManager {}

impl ListenerManager {
    pub(crate) fn create_listener(ty: &str, h: TcpInboundHandler) {
        match ty {
            "tls" => {}
            x => {
                panic!("unknown listener");
            }
        }
    }
}
pub struct NetOpts {
    pub ipv6_only: bool,
}
fn create_dualstack_stream_listener(addr: SocketAddr, opt: NetOpts) -> io::Result<TcpListener> {
    let mut sock = socket2::Socket::new(Domain::IPV4, Type::STREAM, None)?;
    sock.bind(&addr.into())?;
    set_sock_opt(&mut sock)?;
    TcpListener::from_std(sock.into())
}

fn set_sock_opt(sock: &mut Socket) -> io::Result<()> {
    sock.set_nonblocking(true)?;
    sock.set_reuse_port(true)?;
    sock.set_reuse_address(true)?;
    sock.set_nodelay(true)?;
    Ok(())
}

fn create_udp_listener() {}
