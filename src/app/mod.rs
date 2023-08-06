mod numbers;

use core::fmt;

use serde::Deserialize;

use crate::services::{AnotherDatabase, Database, Service};

#[cfg(test)]
use {
    crate::{services::InMemoryDatabase, telemetry},
    once_cell::sync::Lazy,
};

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

#[derive(Clone, Deserialize)]
pub struct AppConfig<DB>
where
    DB: Database,
{
    pub database: DB::Config,
}

impl<DB> Service for GenericApp<DB>
where
    DB: Database,
{
    type Config = AppConfig<DB>;

    fn new(config: Self::Config) -> anyhow::Result<Self> {
        Ok(Self {
            database: DB::new(config.database)?,
        })
    }
}

#[cfg(test)]
static TELEMETRY: Lazy<()> = Lazy::new(|| {
    let level = match std::env::var("LOG_TESTS") {
        Ok(_) => "debug",
        Err(_) => "off",
    };
    std::env::set_var("RUST_LOG", level);
    telemetry::init().unwrap();
});

impl<DB> GenericApp<DB>
where
    DB: Database,
{
    #[cfg(test)]
    pub fn testing() -> TestingApp {
        Lazy::force(&TELEMETRY);
        TestingApp::new(AppConfig::default()).unwrap()
    }
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
