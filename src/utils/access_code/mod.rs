pub mod operation;
pub mod payload;

use sessionless::hex::FromHex;
use sessionless::{PublicKey, Sessionless, Signature};
use operation::*;
use payload::*;
use crate::error::RHSError;
use simple_base64 as base64;
use crate::utils::time;

pub struct AccessCode<'a> {
    pub payload: Payload,
    payload_b64: &'a str,
    sig: Signature,
}

impl<'a> AccessCode<'a> {
    pub fn from_raw(raw: &'a str) -> Result<Self, RHSError> {
        let (payload_raw, sig_raw) = raw
            .split_once('.')
            .ok_or(RHSError::AccessCodeInvalidFormat)?;

        let payload_b64 = serde_json::from_str::<&str>(payload_raw)
            .map_err(|_| RHSError::AccessCodeInvalidFormat)?;

        let payload = serde_json::from_slice::<Payload>(
            &*base64::decode(payload_b64).map_err(|_| RHSError::AccessCodeInvalidFormat)?
        ).map_err(|_| RHSError::AccessCodeInvalidFormat)?;

        let sig = Signature::from_hex(sig_raw)
            .map_err(|_| RHSError::AccessCodeInvalidFormat)?;

        Ok(Self {
            payload,
            payload_b64,
            sig,
        })
    }

    pub fn verify(&self, sessionless: &Sessionless, pub_key: PublicKey) -> Result<(), RHSError> {
        sessionless.verify(
            self.payload_b64,
            &pub_key,
            &self.sig
        ).map_err(|_| RHSError::AccessCodeTampered)?;

        if self.payload.exp < time::now() {
            Err(RHSError::AccessCodeExpired)
        } else {
            Ok(())
        }
    }
}
