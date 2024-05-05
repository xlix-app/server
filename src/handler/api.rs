mod core;
mod version;
mod health;
mod file;

use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use hyper::body::Incoming;
use hyper::header::AUTHORIZATION;
use hyper::http::request::Parts;
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use serde::Serialize;
use core::*;
use crate::utils::AsRes;
use crate::error::RHSError;
use super::*;

lazy_static! {
    /// Lazy loaded map of API endpoint handlers.
    ///
    /// It could be a good idea to force the initialization on startup,
    /// or somehow replace it with different technique.
    static ref API_ENDPOINTS: HashMap<&'static str, Box<dyn API + Sync>> = {
        let mut map = HashMap::new();

        map.insert("/version", version::Version.into_obj());
        map.insert("/health", health::Health.into_obj());
        map.insert("/file", file::File.into_obj());

        map
    };
}

/// This function will call a specific API endpoint handler based on the given `api_path`.
pub async fn handle_api_endpoint(api_path: &str, req: Req, addr: SocketAddr) -> Res {
    if let Some(api_endpoint) = API_ENDPOINTS.get(api_path) {
        let (meta, body) = req.into_parts();
        api_endpoint.handle(meta, body, addr).await.unwrap_or_else(|res| res)
    } else {
        debug!("Endpoint [{}] not found!", api_path);

        RHSError::InvalidEndpoint {
            endpoint: api_path.to_string().into(),
        }.as_res()
    }
}
