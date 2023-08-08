use crate::{
    services::{Database, InMemoryDatabase, Service},
    telemetry,
};
use once_cell::sync::Lazy;

use super::{AppConfig, GenericApp};

pub type TestApp = GenericApp<InMemoryDatabase>;

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
    pub fn testable() -> TestApp {
        Lazy::force(&TELEMETRY);
        TestApp::new(AppConfig::default()).unwrap()
    }
}
