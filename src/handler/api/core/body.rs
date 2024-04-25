use super::*;

pub async fn read_body(mut body: Incoming, max_size: usize) -> Result<Vec<u8>, RHSError> {
    use http_body_util::BodyExt;

    let mut buffer = Vec::new();

    while let Some(frame) = body.frame().await {
        let frame = frame.map_err(|_| RHSError::RejectedBody)?;
        if let Ok(data) = frame.into_data() {
            if buffer.len() + data.len() > max_size {
                return Err(RHSError::RejectedBody);
            }
            buffer.extend(data.to_vec());
        }
    }

    Ok(buffer)
}

pub async fn read_body_json<T: DeserializeOwned>(body: Incoming, max_size: usize) -> Result<T, RHSError> {
    let body = read_body(body, max_size).await?;
    let obj: T = serde_json::from_slice(&body).map_err(|_| RHSError::InvalidBody)?;
    Ok(obj)
}
