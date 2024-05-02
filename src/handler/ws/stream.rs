use std::pin::Pin;
use std::task::{Context, Poll};
use super::*;
use hyper_util::rt::TokioIo;
use tokio_tungstenite::tungstenite::protocol::Role;

type WebSocketStream = tokio_tungstenite::WebSocketStream<TokioIo<hyper::upgrade::Upgraded>>;

pub struct WebSocket {
    on_upgrade: Pin<Box<OnUpgrade>>,
}

impl WebSocket {
    pub(super) fn new(on_upgrade: OnUpgrade) -> Self {
        Self {
            on_upgrade: Box::pin(on_upgrade),
        }
    }
}

impl std::future::Future for WebSocket {
    type Output = Result<WebSocketStream, RHSError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let upgraded = match self.on_upgrade.as_mut().poll(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(upgraded) => upgraded,
        };

        let upgraded = upgraded.map_err(|_| RHSError::HandshakeIncomplete)?;

        let stream = WebSocketStream::from_raw_socket(
            TokioIo::new(upgraded),
            Role::Server,
            None,
        );

        tokio::pin!(stream);

        match stream.as_mut().poll(cx) {
            Poll::Pending => Poll::Ready(Err(RHSError::Unexpected)),
            Poll::Ready(x) => Poll::Ready(Ok(x)),
        }
    }
}
