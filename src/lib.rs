mod adapters;
mod api;
mod app;
mod config;
mod domain;
mod ports;

#[cfg(test)]
mod test_app;

pub mod telemetry;

pub use self::config::init_config;
pub use api::HttpServer;

pub type App = app::App<adapters::PostgresqlDatabase>;
