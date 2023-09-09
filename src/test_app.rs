use crate::{
    adapters::InMemoryDatabase,
    app::{App, AppConfig},
    ports::Database,
    telemetry,
};

type TestApp = App<InMemoryDatabase>;

impl<DB> App<DB>
where
    DB: Database,
{
    pub async fn test() -> TestApp {
        telemetry::init_test();
        let config = AppConfig { database: () };
        TestApp::new(config).await.unwrap()
    }
}
