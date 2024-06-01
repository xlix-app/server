use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tokio::sync::RwLock;
#[cfg(feature = "production")]
use xlix_storage_prod::*;
use xlix_storage::*;
use super::*;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct StorageConfig {
    workers: Vec<String>,
    #[cfg(feature = "production")]
    #[serde(skip)]
    manager: Arc<RwLock<Manager<Worker>>>,
}

impl StorageConfig {
    pub(super) async fn fix(&mut self) -> anyhow::Result<()> {
        let mut manager = self.manager.write().await;

        #[cfg(feature = "production")]
        for worker in std::mem::take(&mut self.workers) {
            let worker = Worker::new(worker);
            manager.add_worker(worker);
        }

        Ok(())
    }

    pub async fn get_worker_next(&self) -> Option<Arc<Worker>> {
        self.manager.read().await.get_worker_next()
    }
}

impl Debug for StorageConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StorageConfig")
            .field("workers", &self.workers)
            .finish()
    }
}
