use color_eyre::eyre::Result;

pub fn tracing() -> Result<()> {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(true).pretty();
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("warn"))?;

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
    Ok(())
}
