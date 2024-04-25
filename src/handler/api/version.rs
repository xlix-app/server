use crate::utils::consts::VERSION;
use super::*;

pub struct Version;

impl API for Version {
    fn handle(&self, _meta: Parts, _body: Incoming, _addr: SocketAddr) -> ResFuture {
        let fut = async move {
            Ok(VERSION.into_res())
        };

        ResFuture {
            handler: Box::pin(fut),
        }
    }
}
