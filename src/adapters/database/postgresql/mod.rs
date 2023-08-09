mod users;

use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    Pool, Postgres,
};

use crate::ports::{Database, Service};

#[derive(Clone)]
pub struct PostgresqlDatabase {
    pool: Pool<Postgres>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostgresqlConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: Secret<String>,
    pub database: String,
    pub require_ssl: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostgresqlDatabaseConfig {
    pub postgresql: PostgresqlConfig,
}

impl Service for PostgresqlDatabase {
    type Config = PostgresqlDatabaseConfig;

    fn new(config: Self::Config) -> anyhow::Result<Self> {
        let config = config.postgresql;
        let options = PgConnectOptions::new()
            .host(&config.host)
            .port(config.port)
            .username(&config.user)
            .password(config.password.expose_secret())
            .database(&config.database)
            .ssl_mode(if config.require_ssl {
                PgSslMode::Require
            } else {
                PgSslMode::Prefer
            });
        Ok(Self {
            pool: Pool::connect_lazy_with(options),
        })
    }
}

impl Database for PostgresqlDatabase {}
