mod stream;

pub use stream::*;

use hyper::upgrade::OnUpgrade;
use tokio_tungstenite::tungstenite::handshake::derive_accept_key;
use log::debug;
use crate::error::RHSError;
use super::*;

const VERSION: &str = "13";

pub fn is_upgrade(req: &Req) -> bool {
    let mut check = req
        .headers()
        .get(hyper::header::CONNECTION)
        .map(|value| value.to_str().ok())
        .unwrap_or_default()
        .map(|connection| connection == "Upgrade")
        .unwrap_or_default();

    if !check {
        return false;
    }

    check = req
        .headers()
        .get(hyper::header::UPGRADE)
        .map(|value| value.to_str().ok())
        .unwrap_or_default()
        .map(|connection| connection == "websocket")
        .unwrap_or_default();

    check
}

pub fn upgrade(req: Req) -> Result<(Res, WebSocket), RHSError> {
    let key = req
        .headers()
        .get("Sec-WebSocket-Key")
        .ok_or(RHSError::MissingSecWebSocketKey)?
        .as_bytes();

    let version = req
        .headers()
        .get("Sec-WebSocket-Version")
        .map(|value| value.to_str().ok())
        .unwrap_or_default()
        .ok_or(RHSError::MissingSecWebSocketVersionHeader)?;

    if version != VERSION {
        return Err(RHSError::DisallowedWebSocketVersion {
            supported_version: VERSION.into(),
        });
    }

    let response = Response::builder()
        .status(hyper::StatusCode::SWITCHING_PROTOCOLS)
        .header(hyper::header::CONNECTION, "upgrade")
        .header(hyper::header::UPGRADE, "websocket")
        .header("Sec-WebSocket-Accept", &derive_accept_key(key))
        .body(Full::<Bytes>::from("switching to websocket protocol"))
        .map_err(|_| RHSError::Unexpected)?;


    Ok((
        response,
        WebSocket::new(hyper::upgrade::on(req)),
    ))
}

pub async fn handle(on_upgrade: WebSocket, addr: SocketAddr) -> Result<(), anyhow::Error> {
    use futures::stream::StreamExt;
    let mut stream = on_upgrade.await?;

    while let Some(message) = stream.next().await {
        match message? {
            msg => debug!("[{}] WebSocket: {}", addr, msg),
        }
    }

    Ok(())
}
