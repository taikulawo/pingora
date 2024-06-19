use std::{
    io,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use async_trait::async_trait;
use log::debug;
use pingora::{
    http::RequestHeader, http_proxy_service_with_name, modules::http::{HttpModule, HttpModuleBuilder, HttpModules}, protocols::{
        raw_connect::ProxyDigest, GetProxyDigest, GetSocketDigest, GetTimingDigest, Shutdown,
        SocketDigest, Ssl, TimingDigest, ALPN,
    }, proxy::{http_proxy_service_with_name, HttpProxy, ProxyHttp, Session}, server::configuration::ServerConf, services::Service, upstreams::peer::HttpPeer
};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

use super::Stream;

struct PingoraStream<T> {
    s: T,
}

impl<T> PingoraStream<T> {
    pub fn new(inner: T) -> Self {
        Self { s: inner }
    }
}

impl<T> Ssl for PingoraStream<T> {}
#[async_trait]
impl<T> Shutdown for PingoraStream<T>
where
    T: Stream,
{
    async fn shutdown(&mut self) {
        AsyncWriteExt::shutdown(self).await.unwrap_or_else(|e| {
            debug!("Failed to shutdown connection: {:?}", e);
        });
    }
}

impl<T> GetTimingDigest for PingoraStream<T>
where
    T: Stream,
{
    fn get_timing_digest(&self) -> Vec<Option<TimingDigest>> {
        let mut digest = Vec::with_capacity(2); // expect to have both L4 stream and TLS layer
        digest.push(Some(TimingDigest {
            established_ts: self.established_ts,
        }));
        digest
    }
}

impl<T> GetProxyDigest for PingoraStream<T>
where
    T: Stream,
{
    fn get_proxy_digest(&self) -> Option<Arc<ProxyDigest>> {
        self.proxy_digest.clone()
    }

    fn set_proxy_digest(&mut self, digest: ProxyDigest) {
        self.proxy_digest = Some(Arc::new(digest));
    }
}

impl<T> GetSocketDigest for PingoraStream<T> {
    fn get_socket_digest(&self) -> Option<Arc<SocketDigest>> {
        self.socket_digest.clone()
    }

    fn set_socket_digest(&mut self, socket_digest: SocketDigest) {
        self.socket_digest = Some(Arc::new(socket_digest))
    }
}

impl<T> AsyncWrite for PingoraStream<T>
where
    T: Stream,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        todo!()
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        todo!()
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        todo!()
    }
}

impl<T> AsyncRead for PingoraStream<T>
where
    T: Stream,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        todo!()
    }
}

struct HttpServer {}
struct ServerCtx {}
#[async_trait]
impl ProxyHttp for HttpServer {
    type CTX = ServerCtx;
    /// Define how the `ctx` should be created.
    fn new_ctx(&self) -> Self::CTX {
        Self::CTX {}
    }
    async fn upstream_peer(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        let peer = HttpPeer::new("127.0.0.1:80", false, "example.org".into());
        peer.options.alpn = ALPN::H1;
        Ok(Box::new(peer))
    }
}
struct ChangeOnTheFly {}

impl HttpModule for ChangeOnTheFly {
    fn request_header_filter(&mut self, _req: &mut RequestHeader) -> pingora::Result<()> {
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        todo!()
    }
}
struct ChangeOntheFlyBuilder {}
impl HttpModuleBuilder for ChangeOntheFlyBuilder {
    fn init(&self) -> Module {
        Box::new(ChangeOnTheFly {})
    }

    fn order(&self) -> i16 {
        0
    }
}
pub fn create_pingora_instance<T>(s: T) {
    let mut http_module = HttpModules::new();
    http_module.add_module();
    let mut ctx = http_module.build_ctx();

    let conf = ServerConf {
        ..Default::default()
    };
    let server = HttpServer {};
    let conf = Arc::new(conf);
    let mut proxy = HttpProxy::new(server, conf.clone());
    let svc = http_proxy_service_with_name(&conf, server, "");
    if let Some(app) = svc.app_logic_mut() {
        let fly = Box::new(ChangeOntheFlyBuilder {});
        app.downstream_modules.add_module(fly)
    }
}