use std::sync::Arc;

use anyhow::Context;
use {{crate_name}}::{config::Config, telemetry};
use structopt::StructOpt;
use tracing::info;

/// A basic async program using Tokio
#[derive(Debug, StructOpt)]
struct Args {
    /// Config file path
    #[structopt(short, long, default_value = "config.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::from_args();

    let config = Config::from_file(&args.config)
        .await
        .context("Failed to get the config")?;
    let config = Arc::new(config);

    telemetry::init(&config.trace_level);

    info!("Success ✅");

    Ok(())
}
