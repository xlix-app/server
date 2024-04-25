use hyper::header::AsHeaderName;
use super::*;

#[derive(Copy, Clone, Debug)]
pub enum Action {
    GET,
    POST,
    PATCH,
    DELETE,
    ARCHIVE,
}

impl Action {
    pub fn from_meta(meta: &Parts) -> Result<Self, RHSError> {
        let action = get_header(meta, "action").trim();

        if action.is_empty() {
            return Ok(Self::GET);
        }

        match action.trim() {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PATCH" => Ok(Self::PATCH),
            "DELETE" => Ok(Self::DELETE),
            "ARCHIVE" => Ok(Self::ARCHIVE),
            _ => Err(RHSError::InvalidHeader {
                header: "action".into(),
            })
        }
    }
}

pub fn get_header<T: AsHeaderName>(meta: &Parts, header: T) -> &str {
    meta.headers
        .get(header)
        .map(|val| val.to_str().unwrap_or_default())
        .unwrap_or_default()
}

pub fn get_header_try<T: AsHeaderName>(meta: &Parts, header: T) -> Option<&str> {
    meta.headers
        .get(header)
        .map(|val| val.to_str().ok())?
}
