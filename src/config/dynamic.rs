use tokio::sync::{OnceCell, RwLock, RwLockReadGuard};
use super::*;

static CFG: OnceCell<RwLock<ConfigDyn>> = OnceCell::const_new();

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ConfigDyn {

}

impl CfgIntern for ConfigDyn {
    fn path() -> &'static str {
        "config/dynamic.json"
    }
}

impl Cfg for ConfigDyn {
    type Config = RwLockReadGuard<'static, ConfigDyn>;
    type Error = ();

    async fn init() -> anyhow::Result<()> {
        let initialization = async move {
            let cfg = ConfigDyn::load().await?;
            Ok(RwLock::new(cfg))
        };

        CFG.get_or_try_init(|| initialization)
            .await
            .map(|_| ())
    }

    async fn get() -> Result<Self::Config, Self::Error> {
        if let Some(lock) = CFG.get() {
            Ok(lock.read().await)
        } else {
            Err(())
        }
    }
}
