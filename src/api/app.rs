use serde::Deserialize;

use crate::adapters::mysql::{MySqlConfig, MySqlDatabase};

pub type App = crate::app::App<MySqlDatabase>;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub mysql: MySqlConfig,
}

impl App {
    #[tracing::instrument]
    pub async fn new(config: AppConfig) -> anyhow::Result<Self> {
        Ok(Self::with(MySqlDatabase::new(config.mysql).await?))
    }
}
