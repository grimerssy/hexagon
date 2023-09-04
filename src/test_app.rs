use crate::{
    adapters::InMemoryDatabase,
    app::{App, AppConfig},
    ports::Database,
};
use once_cell::sync::Lazy;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;

type TestApp = App<InMemoryDatabase>;

static TELEMETRY: Lazy<()> = Lazy::new(init_telemetry);

impl<DB> App<DB>
where
    DB: Database,
{
    pub async fn test() -> TestApp {
        Lazy::force(&TELEMETRY);
        let config = AppConfig { database: () };
        TestApp::new(config).await.unwrap()
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
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .pretty()
        .init()
}
