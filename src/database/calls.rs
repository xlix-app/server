use sessionless::hex::IntoHex;
use sessionless::PublicKey;
use crate::error::RHSError;
use super::*;

impl Database {
    pub async fn system_create(&self, name: impl AsRef<str>, public_key: Option<PublicKey>) -> Result<table_system::Object, RHSError> {
        let public_key = public_key.map(|key| key.into_hex());

        let mut response = self
            .query(surql::SYSTEM_CREATE)
            .bind(("name", name.as_ref()))
            .bind(("public_key", public_key))
            .await?;

        let system = response
            .take::<Option<table_system::Object>>(0)?
            .ok_or_else(|| RHSError::DatabaseFailure {
                msg: Some("Failed to create the system!".into()),
            })?;

        Ok(system)
    }

    pub async fn system_get_by_name(&self, system: impl AsRef<str>) -> Result<table_system::Object, RHSError> {
        let mut response = self
            .query(surql::SYSTEM_GET_BY_NAME)
            .bind(("system_name", system.as_ref()))
            .await?;

        let system = response
            .take::<Option<table_system::Object>>(0)?
            .ok_or_else(|| RHSError::EntityNotFound {
                entity: system.as_ref().into(),
            })?;

        Ok(system)
    }
}
