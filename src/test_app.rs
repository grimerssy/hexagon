use crate::{
    app::{AppConfig, GenericApp},
    services::{Database, InMemoryDatabase, Service},
};
use once_cell::sync::Lazy;
use tracing::level_filters::LevelFilter;

type TestApp = GenericApp<InMemoryDatabase>;

static TELEMETRY: Lazy<()> = Lazy::new(init_telemetry);

impl<DB> GenericApp<DB>
where
    DB: Database,
{
    pub fn testable() -> TestApp {
        Lazy::force(&TELEMETRY);
        TestApp::new(AppConfig::default()).unwrap()
    }
}

fn init_telemetry() {
    let verbosity_level = if cfg!(feature = "log-tests") {
        LevelFilter::DEBUG
    } else {
        LevelFilter::OFF
    };
    tracing_subscriber::fmt()
        .with_max_level(verbosity_level)
        .pretty()
        .init()
}
