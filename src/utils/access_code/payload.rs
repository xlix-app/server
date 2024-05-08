use super::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
#[serde(untagged)]
pub enum Payload {
    CreateFileRequest {
        sys: String,
        sub: String,
        size: u64,
        exp: u64,
    },
    TransferDataTicket {
        file_id: String,
        exp: u64,
    },
}

impl Payload {
    pub fn get_exp(&self) -> Option<u64> {
        use Payload::*;

        match self {
            CreateFileRequest { exp, .. } |
            TransferDataTicket { exp, .. } => Some(*exp),
            _ => None,
        }
    }
}

// #[derive(Serialize, Deserialize)]
// #[serde(rename_all="camelCase")]
// pub struct Payload {
//     pub id: String,
//     pub op: Operation,
//     pub sys: String,
//     pub exp: u64,
// }
