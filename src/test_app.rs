use crate::{
    adapters::InMemoryDatabase,
    app::{App, AppConfig},
    ports::Database,
    telemetry::init_test_telemetry,
};

type TestApp = App<InMemoryDatabase>;

impl<DB> App<DB>
where
    DB: Database,
{
    pub async fn test() -> TestApp {
        init_test_telemetry();
        let config = AppConfig { database: () };
        TestApp::new(config).await.unwrap()
    }
}
