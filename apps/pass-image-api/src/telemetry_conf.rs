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
fn init_tracer() -> opentelemetry_sdk::trace::SdkTracerProvider {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .expect("Failed to create span exporter");

    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(get_resource())
        .build();

    global::set_tracer_provider(tracer_provider.clone());
    tracer_provider
}

// A Meter Provider is a factory for Meters
// A Meter creates metric instruments, capturing measurements about a service at runtime.
fn init_meter_provider() -> Result<opentelemetry_sdk::metrics::SdkMeterProvider> {
    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_temporality(opentelemetry_sdk::metrics::Temporality::Delta)
        .build()
        .with_context(|| "creating metric exporter")?;

    let meter_provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder()
        .with_reader(opentelemetry_sdk::metrics::PeriodicReader::builder(exporter).build())
        .with_resource(get_resource())
        .build();

    global::set_meter_provider(meter_provider.clone());

    Ok(meter_provider)
}

// A Logger Provider is a factory for Loggers
// The init_logger_provider function initialises a Logger Provider
// And sets up a Log Appender for the log crate, bridging logs to the OpenTelemetry Logger.
// If OTLP setup fails, falls back to stdout logging.
fn init_logger_provider() -> Option<opentelemetry_sdk::logs::SdkLoggerProvider> {
    // Read maximum log level from the enironment, using INFO if it's missing or
    // we can't parse it.
    let max_level = env::var("LOG_LEVEL")
        .ok()
        .and_then(|l| Level::from_str(l.to_lowercase().as_str()).ok())
        .unwrap_or(Level::Info);

    // Try to setup OTLP logging first
    match opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .build()
    {
        Ok(exporter) => {
            let logger_provider = opentelemetry_sdk::logs::SdkLoggerProvider::builder()
                .with_batch_exporter(exporter)
                .with_resource(get_resource())
                .build();

            // Setup Log Appender for the log crate
            let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
            log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
            log::set_max_level(max_level.to_level_filter());

            Some(logger_provider)
        }
        Err(err) => {
            // Fall back to stdout logging if OTLP setup fails
            env_logger::Builder::from_default_env()
                .filter_level(max_level.to_level_filter())
                .init();

            log::warn!(
                "Failed to initialize OTLP log exporter: {}. Falling back to stdout logging.",
                err
            );

            None
        }
    }
}

pub fn init_otel() -> Result<(
    opentelemetry_sdk::trace::SdkTracerProvider,
    opentelemetry_sdk::metrics::SdkMeterProvider,
    Option<opentelemetry_sdk::logs::SdkLoggerProvider>,
)> {
    let logger_provider = init_logger_provider();
    let tracer_provider = init_tracer();
    let meter_provider = init_meter_provider().with_context(|| "initialising meter provider")?;

    Ok((tracer_provider, meter_provider, logger_provider))
}
