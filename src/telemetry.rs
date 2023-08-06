use anyhow::Context;
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter, FmtSubscriber};

pub fn init() -> anyhow::Result<()> {
    LogTracer::init()?;
    let filter = EnvFilter::try_from_default_env().unwrap_or("info".into());
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .pretty()
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to set global log subscriber")
}

pub(crate) async fn instrument_blocking<F, R>(f: F) -> anyhow::Result<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
        .await
        .context("Failed to spawn blocking task")
}
