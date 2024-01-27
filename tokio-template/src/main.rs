use std::sync::Arc;

use anyhow::Context;
use config::Config;
use opentelemetry_sdk::{propagation::TraceContextPropagator, runtime::Tokio};
use structopt::StructOpt;

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
    init_tracing();

    let args = Args::from_args();

    let config = Config::from_file(&args.config)
        .await
        .context("Failed to get the config")?;
    let config = Arc::new(config);

    println!("Success ✅");

    Ok(())
}

fn init_tracing() {
    use tracing_subscriber::prelude::*;

    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::default());

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("scraper")
        .install_batch(Tokio)
        .expect("Error initializing Jaeger exporter");

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let fmt_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(fmt_layer)
        .with(telemetry_layer)
        .init();
}
