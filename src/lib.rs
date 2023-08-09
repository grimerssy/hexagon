mod adapters;
mod api;
mod app;
mod config;
mod domain;
mod ports;
mod telemetry;

#[cfg(test)]
mod test_app;

pub use self::config::init_config;
pub use api::HttpServer;
pub use telemetry::init as init_telemetry;

pub type App = app::App<adapters::PostgresqlDatabase>;
