use http_body_util::Full;
use hyper::body::Bytes;
use hyper::http::response;
use crate::handler::Res;
use crate::utils::AsRes;
use super::*;

impl AsRes for RHSError {
    fn as_res(&self) -> Res {
        let body = Full::new(Bytes::from(
            serde_json::to_string(self).unwrap()
        ));

        let builder = response::Builder::new()
            .status(self.get_http_code())
            .body(body);

        // shouldn't panic
        builder.unwrap()
    }
}
