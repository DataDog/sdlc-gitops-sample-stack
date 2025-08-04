use anyhow::{Context, Result};
use log::Level;
use opentelemetry::global;
use opentelemetry_appender_log::OpenTelemetryLogBridge;

use opentelemetry_resource_detectors::{OsResourceDetector, ProcessResourceDetector};
use opentelemetry_sdk::{propagation::TraceContextPropagator, Resource};

use std::{env, str::FromStr};

// get_resource returns a Resource containing information about the environment
// The Resource is used to provide context to Traces, Metrics and Logs
// It is created by merging the results of multiple ResourceDetectors
// The ResourceDetectors are responsible for detecting information about the environment
fn get_resource() -> Resource {
    Resource::builder()
        .with_service_name("pass-image-api") // Default service name, can be overridden by OTEL_SERVICE_NAME env var
        .with_detector(Box::new(OsResourceDetector))
        .with_detector(Box::new(ProcessResourceDetector))
        .build()
}

// A Tracer Provider is a factory for Tracers
// A Tracer creates spans containing more information about what is happening for a given operation,
// such as a request in a service.
fn init_tracer() {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .expect("Failed to create span exporter");

    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(get_resource())
        .build();

    global::set_tracer_provider(tracer_provider);
}

// A Meter Provider is a factory for Meters
// A Meter creates metric instruments, capturing measurements about a service at runtime.
fn init_meter_provider() -> Result<()> {
    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_temporality(opentelemetry_sdk::metrics::Temporality::Delta)
        .build()
        .with_context(|| "creating metric exporter")?;

    let meter_provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder()
        .with_reader(opentelemetry_sdk::metrics::PeriodicReader::builder(exporter).build())
        .with_resource(get_resource())
        .build();

    global::set_meter_provider(meter_provider);

    Ok(())
}

// A Logger Provider is a factory for Loggers
// The init_logger_provider function initialises a Logger Provider
// And sets up a Log Appender for the log crate, bridging logs to the OpenTelemetry Logger.
fn init_logger_provider() {
    let exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .build()
        .expect("Failed to create log exporter");

    let logger_provider = opentelemetry_sdk::logs::SdkLoggerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(get_resource())
        .build();

    // Setup Log Appender for the log crate
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();

    // Read maximum log level from the enironment, using INFO if it's missing or
    // we can't parse it.
    let max_level = env::var("LOG_LEVEL")
        .ok()
        .and_then(|l| Level::from_str(l.to_lowercase().as_str()).ok())
        .unwrap_or(Level::Info);
    log::set_max_level(max_level.to_level_filter());
}

pub fn init_otel() -> Result<()> {
    init_logger_provider();
    init_tracer();
    init_meter_provider().with_context(|| "initialising meter provider")?;
    Ok(())
}
