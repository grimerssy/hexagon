mod users;

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
    #[tracing::instrument(skip(config))]
    pub async fn new(config: AppConfig<DB>) -> anyhow::Result<Self> {
        Ok(Self {
            database: DB::new(config.database).await?,
        })
    }
}
