use opentelemetry_sdk::{propagation::TraceContextPropagator, runtime::Tokio};
use std::str::FromStr;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

pub fn init(tracing_level: &str) {
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::default());

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name("hello-tokio")
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
