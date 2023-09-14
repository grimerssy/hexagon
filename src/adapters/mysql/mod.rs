mod users;

use anyhow::Context;
use async_trait::async_trait;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlSslMode},
    MySql, Pool,
};

use crate::ports::{Database, Service};

#[derive(Clone)]
pub struct MySqlDatabase {
    pool: Pool<MySql>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MySqlConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: Secret<String>,
    pub database: String,
    pub require_ssl: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MySqlDatabaseConfig {
    pub mysql: MySqlConfig,
}

#[async_trait]
impl Service for MySqlDatabase {
    type Config = MySqlDatabaseConfig;

    async fn new(config: Self::Config) -> anyhow::Result<Self> {
        let config = config.mysql;
        let options = MySqlConnectOptions::new()
            .host(&config.host)
            .port(config.port)
            .username(&config.user)
            .password(config.password.expose_secret())
            .database(&config.database)
            .ssl_mode(if config.require_ssl {
                MySqlSslMode::Required
            } else {
                MySqlSslMode::Preferred
            });
        let pool = Pool::connect_with(options)
            .await
            .context("Failed to connect to the database")?;
        Ok(Self { pool })
    }
}

impl Database for MySqlDatabase {}
