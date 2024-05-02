pub mod api;
mod resources;
mod ws;

use std::convert::Infallible;
use std::net::SocketAddr;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response};

pub(crate) use resources::reload_resource_map;
use crate::utils::AsRes;

pub type Req = Request<hyper::body::Incoming>;
pub type Res = Response<Full<Bytes>>;

const HTML_FILE_EXTENSION: &str = ".html";

/// Main HTTP service. Each request made to the server lands here.
///
/// # Errors
/// This function should always return a Response in an `Ok` variant!
pub async fn service(req: Req, addr: SocketAddr) -> Result<Res, Infallible> {
    if ws::is_upgrade(&req) {
        let res = match ws::upgrade(req) {
            Ok((res, web_socket)) => {
                tokio::spawn(async move {
                    if let Err(err) = ws::handle(web_socket, addr).await {
                        error!("[{}] WebSocket: {}", addr, err);
                    }
                });

                res
            },
            Err(err) => err.into_res(),
        };

        return Ok(res);
    }

    let req_path = req.uri().path().to_owned();
    let mut path = req_path.as_str();

    // This will help support local HTML links
    if path.ends_with(HTML_FILE_EXTENSION) {
        path = path.rsplit_once(HTML_FILE_EXTENSION).unwrap().0;
    }

    let mut res: Res;

    // API based requests
    if path.starts_with("/api/") {
        let api_path = path.split_once("/api").unwrap().1;
        res = api::handle_api_endpoint(api_path, req, addr).await;

        res.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        return Ok(res);
    }

    // Resource based requests
    res = resources::handle_resource_endpoint(path, req).await;

    Ok(res)
}
