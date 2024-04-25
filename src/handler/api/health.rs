use hyper::http::response::Builder;
use super::*;

pub struct Health;

impl API for Health {
    fn handle(&self, _meta: Parts, _body: Incoming, _addr: SocketAddr) -> ResFuture {
        let fut = async move {
            let res = Builder::new()
                .status(204)
                .body(Full::new(Bytes::new()))
                .unwrap();

            Ok(res)
        };

        ResFuture {
            handler: Box::pin(fut),
        }
    }
}
