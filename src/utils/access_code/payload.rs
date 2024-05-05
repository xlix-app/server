use super::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Payload {
    pub op: Operation,
    pub sys: String,
    pub exp: u64,
}
