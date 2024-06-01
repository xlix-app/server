mod dynamic;
mod persistent;

use std::io::{Error, ErrorKind};
use std::path::Path;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tokio::fs::{create_dir_all, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
pub use dynamic::*;
pub use persistent::*;

trait CfgIntern: for<'a> Deserialize<'a> + Serialize + Default {
    fn path() -> &'static str;

    async fn fix(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    async fn load() -> anyhow::Result<Self> {
        let config_path = Path::new(Self::path());

        let mut config = if !config_path.is_file() {
            warn!("Missing config file [{}], creating a default one.", Self::path());
            let config = Self::default();

            if let Some(dir_path) = config_path.parent() {
                let _ = create_dir_all(dir_path).await;
            }

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(config_path)
                .await?;

            file.write_all(
                serde_json::to_string_pretty(&config)?.as_bytes()
            ).await?;

            config
        } else {
            let mut buffer = String::new();

            OpenOptions::new()
                .read(true)
                .open(config_path)
                .await?
                .read_to_string(&mut buffer)
                .await?;

            match serde_json::from_str::<Self>(buffer.as_str()) {
                Ok(config) => config,
                Err(err) => return Err(
                   anyhow!("Invalid JSON structure for the config file [{}]: {}", Self::path(), err)
                ),
            }
        };

        config.fix().await?;

        Ok(config)
    }
}

pub trait Cfg: CfgIntern {
    type Config;
    type Error;

    async fn init() -> anyhow::Result<()>;

    async fn get() -> Result<Self::Config, Self::Error>;
}
