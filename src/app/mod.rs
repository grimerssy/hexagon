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

#[derive(Clone, Debug, Deserialize)]
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
    pub fn new(config: AppConfig<DB>) -> anyhow::Result<Self> {
        Ok(Self {
            database: DB::new(config.database)?,
        })
    }
}

// Workaround to implement `Default` trait for app config.
// This is done in such a way so:
// - generic types do not have to implement `Default` as they are not used directly;
// - if config types are not `Default`, this implementation will just be skipped.
impl<DB> Default for AppConfig<DB>
where
    DB: Database,
    DB::Config: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            database: Default::default(),
        }
    }
}
