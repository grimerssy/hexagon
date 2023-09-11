pub mod api;
pub mod config;
pub mod telemetry;

pub type App = app::App<adapters::PostgresqlDatabase>;

mod adapters;
mod app;
mod domain;
mod ports;

#[cfg(test)]
mod test_app;
