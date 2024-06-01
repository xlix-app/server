pub mod storage;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::sync::OnceCell;
use super::*;

static CFG: OnceCell<ConfigPer> = OnceCell::const_new();

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigPer {
    pub addr_server: SocketAddr,
    pub db_update_interval: u64,
    pub storage: storage::StorageConfig,
}

impl Default for ConfigPer {
    fn default() -> Self {
        Self {
            addr_server: SocketAddr::new(
                IpAddr::V4(Ipv4Addr::LOCALHOST),
                80,
            ),
            db_update_interval: 10,
            storage: Default::default(),
        }
    }
}

impl CfgIntern for ConfigPer {
    fn path() -> &'static str {
        "config/persistent.json"
    }

    async fn fix(&mut self) -> anyhow::Result<()> {
        if self.db_update_interval < 5 {
            warn!("Config value 'db_update_interval' under the minimum value, updating to: '5'.");
            self.db_update_interval = 5;
        }

        self.storage.fix().await?;

        Ok(())
    }
}

impl Cfg for ConfigPer {
    type Config = &'static ConfigPer;
    type Error = ();

    async fn init() -> anyhow::Result<()> {
        let initialization = async move {
            ConfigPer::load().await.map_err(anyhow::Error::from)
        };

        CFG.get_or_try_init(|| initialization)
            .await
            .map(|_| ())
    }

    async fn get() -> Result<Self::Config, Self::Error> {
        CFG.get().ok_or_else(|| ())
    }
}
