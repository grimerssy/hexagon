mod numbers;

use core::fmt;

use serde::Deserialize;

#[cfg(test)]
use crate::services::InMemoryDatabase;
use crate::services::{AnotherDatabase, Database, Service};

pub type App = GenericApp<AnotherDatabase>;
#[cfg(test)]
type TestingApp = GenericApp<InMemoryDatabase>;

#[derive(Clone)]
pub struct GenericApp<DB>
where
    DB: Database,
{
    database: DB,
}

impl<DB> Service for GenericApp<DB>
where
    DB: Database,
{
    type Config = AppConfig<DB>;

    fn new(config: Self::Config) -> anyhow::Result<Self> {
        // TODO telemetry
        // maybe not here, don't want to send anything to jaeger in test mode
        Ok(Self {
            database: DB::new(config.database)?,
        })
    }
}

impl<DB> GenericApp<DB>
where
    DB: Database,
{
    #[cfg(test)]
    pub fn testing() -> TestingApp {
        TestingApp::new(AppConfig::default()).unwrap()
    }
}

#[derive(Clone, Deserialize)]
pub struct AppConfig<DB>
where
    DB: Database,
{
    pub database: DB::Config,
}

// Workaround to implement `Debug` and `Default` for config
// #[derive(Debug, Default)] requires all generic parameters to also implement it
// even if they are not used directly
impl<DB> Default for AppConfig<DB>
where
    DB: Database,
    DB::Config: Default,
{
    fn default() -> Self {
        Self {
            database: DB::Config::default(),
        }
    }
}
impl<DB> fmt::Debug for AppConfig<DB>
where
    DB: Database,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppConfig {{ database: {:?} }}", self.database)
    }
}
