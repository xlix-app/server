pub(crate) mod surql;
pub mod types;
mod calls;

use std::ops::Deref;
use std::time::Duration;
use anyhow::anyhow;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tokio::sync::OnceCell;
use tokio::time::interval;
use types::*;
use crate::config::{Cfg, ConfigPer};

type SurrealClient = Surreal<Db>;

#[cfg(not(test))]
const DB_PATH: &str = "rhs.db";

#[cfg(not(test))]
static DB_INST: OnceCell<Database> = OnceCell::const_new();

#[derive(Debug, Clone)]
pub struct Database {
    client: SurrealClient,
}

impl Database {
    /// Initializes the database.
    ///
    /// Run only once!
    async fn init() -> anyhow::Result<Self> {
        let db_config = surrealdb::opt::Config::default()
            .strict();

        #[cfg(not(test))]
            let client: SurrealClient = Surreal::new::<surrealdb::engine::local::RocksDb>(
            (DB_PATH, db_config)
        ).await?;

        #[cfg(test)]
            let client: SurrealClient = Surreal::new::<surrealdb::engine::local::Mem>(
            db_config
        ).await?;

        let db = Self { client };
        db.build().await?;

        // if changed update build.surql!
        db.client.use_ns("main").use_db("main").await?;

        #[cfg(not(test))]
        db.spawn_update_task().await?;

        Ok(db)
    }

    async fn spawn_update_task(&self) -> anyhow::Result<()> {
        let db = self.clone();
        let update_interval = ConfigPer::get()
            .await
            .map_err(|_| anyhow!("Config not initialized!"))?
            .db_update_interval;

        tokio::spawn(async move {
            let mut clock = interval(Duration::from_secs(update_interval));
            loop {
                clock.tick().await;
                let result = db
                    .query(surql::UPDATE)
                    .await
                    .map(|result| result.check());

                match result {
                    Ok(Err(err)) | Err(err) => {
                        // todo: Add better logging.
                        error!("DB update task failed with: {}", err);
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    /// Get the database instance or initialize it if not present.
    ///
    /// Can return an error only when creating a new database instance,
    /// so it's safe to unwrap result after calling it once with a success.
    #[cfg(not(test))]
    pub async fn get() -> Result<&'static Database, anyhow::Error> {
        DB_INST.get_or_try_init(|| async move {
            Self::init().await
        }).await
    }

    #[cfg(test)]
    pub async fn get() -> anyhow::Result<Database> {
        Self::init().await
    }

    /// Builds the database by executing the `build` query.
    async fn build(&self) -> anyhow::Result<()> {
        self.query(surql::BUILD).await?.check()?;
        Ok(())
    }
}

impl Deref for Database {
    type Target = SurrealClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
