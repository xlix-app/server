pub mod operation;
pub mod payload;

use sessionless::hex::{FromHex, IntoHex};
use sessionless::{PrivateKey, PublicKey, Sessionless, Signature};
use payload::*;
use crate::error::RHSError;
use simple_base64 as base64;
use crate::utils::{SESSIONLESS, time};

pub struct AccessCode<'a> {
    pub payload: Payload,
    payload_b64: &'a str,
    sig: Signature,
}

impl<'a> AccessCode<'a> {
    pub fn new(payload: Payload, private_key: Option<&str>) -> anyhow::Result<String> {
        let payload_json = serde_json::to_string(&payload)?;
        let payload_b64 = base64::encode(payload_json);
        let sig = if let Some(private_key) = private_key {
            let ctx = Sessionless::from_private_key(PrivateKey::from_hex(private_key)?);
            ctx.sign(&payload_b64)
        } else {
            SESSIONLESS.sign(&payload_b64)
        };

        let access_code = format!("{}.{}", payload_b64, sig.to_hex());

        Ok(access_code)
    }

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

    pub fn verify_lifetime(&self) -> Result<(), RHSError> {
        if let Some(exp) = self.payload.get_exp() {
            if exp < time::now() {
                return Err(RHSError::AccessCodeExpired);
            }
        }

        Ok(())
    }

    pub fn verify(&self, sessionless: &Sessionless, pub_key: PublicKey) -> Result<(), RHSError> {
        sessionless.verify(
            self.payload_b64,
            &pub_key,
            &self.sig
        ).map_err(|_| RHSError::AccessCodeTampered)?;

        Ok(())
    }
}
