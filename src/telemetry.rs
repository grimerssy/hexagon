use anyhow::{anyhow, Context};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

pub fn init() -> anyhow::Result<()> {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("INFO"));
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::ERROR)
        .with_env_filter(env_filter)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .pretty()
        .try_init()
        .map_err(|e| anyhow!("Unable to install global subscriber: {e}"))
}

pub async fn instrument_blocking<F, R>(f: F) -> anyhow::Result<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(|| tracing::Span::current().in_scope(f))
        .await
        .context("Failed to spawn blocking task")
}
