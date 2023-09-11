mod users;

use std::fmt;

use serde::Deserialize;

use crate::ports::Database;

#[derive(Clone)]
pub struct App<DB>
where
    DB: Database,
{
    database: DB,
}

#[derive(Clone, Deserialize)]
pub struct AppConfig<DB>
where
    DB: Database,
{
    #[serde(flatten)]
    pub database: DB::Config,
}

impl<DB> App<DB>
where
    DB: Database,
{
    #[tracing::instrument]
    pub async fn new(config: AppConfig<DB>) -> anyhow::Result<Self> {
        Ok(Self {
            database: DB::new(config.database).await?,
        })
    }
}

impl<DB> fmt::Debug for AppConfig<DB>
where
    DB: Database,
    DB::Config: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppConfig")
            .field("database", &self.database)
            .finish()
    }
}
