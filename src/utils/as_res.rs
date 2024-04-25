use std::borrow::Cow;
use http_body_util::Full;
use hyper::body::Bytes;
use serde::Serialize;
use crate::handler::Res;
use crate::error::RHSError;

/// Converts self into an HTTP JSON Response.
pub trait AsRes: Serialize {
    fn as_res(&self) -> Res {
        Res::new(Full::new(Bytes::from(
            serde_json::to_string(self).unwrap()
        )))
    }

    fn into_res(self) -> Res where Self: Sized {
        self.as_res()
    }
}

impl AsRes for String {}
impl AsRes for &str {}
impl<'a> AsRes for Cow<'a, str> {}

impl<T: AsRes, U: AsRes> AsRes for Result<T, U> {
    fn as_res(&self) -> Res {
        match self {
            Ok(ok) => ok.as_res(),
            Err(err) => err.as_res(),
        }
    }
}

impl From<RHSError> for Res {
    fn from(err: RHSError) -> Self {
        err.as_res()
    }
}
