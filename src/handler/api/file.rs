use hyper::Method;
use sessionless::hex::FromHex;
use sessionless::PublicKey;
use crate::database::Database;
use crate::utils::access_code::AccessCode;
use crate::utils::SESSIONLESS;
use super::*;

#[derive(Deserialize)]
struct FilePostPayload {
    size: usize,
}

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
struct FilePostResponse {
    file_id: String,
}

impl AsRes for FilePostResponse {}

pub struct File;

impl File {
    async fn post(meta: Parts, body: Incoming, addr: SocketAddr) -> Result<Res, RHSError> {
        let access_code_raw = get_header_try(&meta, AUTHORIZATION)
            .ok_or_else(|| RHSError::MissingHeader {
                header: AUTHORIZATION.to_string().into(),
            })?;

        let access_code = AccessCode::from_raw(access_code_raw)?;

        let payload = read_body_json::<FilePostPayload>(body, 10_000).await?;
        // todo: check file size

        let db: Database = Database::get().unwrap();
        let system = db.system_get_by_name(&access_code.payload.sys).await?;

        let system_pub_key = system
            .public_key
            .map(|hex| PublicKey::from_hex(hex.into_bytes()))
            .ok_or(RHSError::SystemHasNoAssignedKeys)?
            .map_err(|_| RHSError::SystemAssignedKeyIsInvalid)?;

        access_code.verify(&*SESSIONLESS, system_pub_key)?;

        let file_id = db.create_file(payload.size).await?;
        // todo: invalidate the access code

        Ok(FilePostResponse {
            file_id: file_id.into_raw(),
        }.into_res())
    }
}

impl API for File {
    fn handle(&self, meta: Parts, body: Incoming, addr: SocketAddr) -> ResFuture {
        let fut = async move {
            match meta.method {
                Method::POST => File::post(meta, body, addr).await.map_err(|err| err.into_res()),
                _ => Err(RHSError::MethodNotAccepted.into_res()),
            }
        };

        ResFuture {
            handler: Box::pin(fut),
        }
    }
}
