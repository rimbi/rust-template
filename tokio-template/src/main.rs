use std::{str::FromStr, sync::Arc};

use anyhow::Context;
use config::Config;
use opentelemetry_sdk::{propagation::TraceContextPropagator, runtime::Tokio};
use structopt::StructOpt;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::prelude::*;

mod config;

/// Scraper app to fetch news from a website
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

    init_tracing(&config.trace_level);

    info!("Success ✅");

    Ok(())
}

fn init_tracing(tracing_level: &str) {
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::default());

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("scraper")
        .install_batch(Tokio)
        .expect("Error initializing Jaeger exporter");

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let fmt_layer = tracing_subscriber::fmt::layer();

    // The filter layer can be set via
    // - RUST_LOG env variable
    // - config file
    // - default value `info`
    // such that the former overrides the latter.
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(tracing_level))
        .or_else(|_| EnvFilter::from_str("info"))
        .expect("Creating the filter layer can not fail!");

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(telemetry_layer)
        .init();
}
