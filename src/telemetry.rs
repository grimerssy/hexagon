//TODO opentelemetry

use anyhow::{anyhow, Context};
use once_cell::sync::OnceCell;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

static TELEMETRY: OnceCell<()> = OnceCell::new();

pub fn init() -> anyhow::Result<()> {
    TELEMETRY.get_or_try_init(|| {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("INFO"));
        tracing_subscriber::fmt()
            .with_max_level(LevelFilter::ERROR)
            .with_env_filter(env_filter)
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .pretty()
            .try_init()
            .map_err(|e| anyhow!("Unable to install global subscriber: {e}"))
    })?;
    Ok(())
}

pub fn init_test() {
    TELEMETRY.get_or_init(|| {
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
    });
}

#[allow(unused)]
pub async fn instrument_blocking<F, R>(f: F) -> anyhow::Result<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(|| tracing::Span::current().in_scope(f))
        .await
        .context("Failed to spawn blocking task")
}

pub fn warn<E: std::fmt::Debug>(e: E) -> E {
    tracing::warn!("{e:?}");
    e
}

pub fn error<E: std::fmt::Debug>(e: E) -> E {
    tracing::error!("{e:?}");
    e
}
