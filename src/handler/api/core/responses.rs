use super::*;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub enum SingleResponse {
    Ok,
}

impl AsRes for SingleResponse {}
