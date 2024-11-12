use anyhow::{Context, Result};
use log::Level;
use opentelemetry::global;
use opentelemetry_appender_log::OpenTelemetryLogBridge;

use opentelemetry_resource_detectors::{OsResourceDetector, ProcessResourceDetector};
use opentelemetry_sdk::{
    propagation::TraceContextPropagator,
    resource::{
        EnvResourceDetector, ResourceDetector, SdkProvidedResourceDetector,
        TelemetryResourceDetector,
    },
    runtime, Resource,
};

use std::time::Duration;

// get_resource returns a Resource containing information about the environment
// The Resource is used to provide context to Traces, Metrics and Logs
// It is created by merging the results of multiple ResourceDetectors
// The ResourceDetectors are responsible for detecting information about the environment
fn get_resource() -> Resource {
    let os_resource = OsResourceDetector.detect(Duration::from_secs(0));
    let process_resource = ProcessResourceDetector.detect(Duration::from_secs(0));
    let sdk_resource = SdkProvidedResourceDetector.detect(Duration::from_secs(0));
    let env_resource = EnvResourceDetector::new().detect(Duration::from_secs(0));
    let telemetry_resource = TelemetryResourceDetector.detect(Duration::from_secs(0));

    os_resource
        .merge(&process_resource)
        .merge(&sdk_resource)
        .merge(&env_resource)
        .merge(&telemetry_resource)
}

// A Tracer Provider is a factory for Tracers
// A Tracer creates spans containing more information about what is happening for a given operation,
// such as a request in a service.
fn init_tracer() -> () {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            opentelemetry_sdk::trace::Config::default().with_resource(get_resource()),
        )
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(runtime::Tokio)
        .expect("Failed to initialise tracing provider");

    global::set_tracer_provider(tracer_provider);
}

// A Meter Provider is a factory for Meters
// A Meter creates metric instruments, capturing measurements about a service at runtime.
fn init_meter_provider() -> Result<()> {
    let meter_provider = opentelemetry_otlp::new_pipeline()
        .metrics(opentelemetry_sdk::runtime::Tokio)
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_resource(get_resource())
        .with_delta_temporality()
        .build()
        .with_context(|| "creating meter provider")?;

    global::set_meter_provider(meter_provider);

    Ok(())
}

// A Logger Provider is a factory for Loggers
// The init_logger_provider function initialises a Logger Provider
// And sets up a Log Appender for the log crate, bridging logs to the OpenTelemetry Logger.
fn init_logger_provider() -> () {
    let logger_provider = opentelemetry_otlp::new_pipeline()
        .logging()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_resource(get_resource())
        .install_batch(runtime::Tokio)
        .expect("Failed to initialise logger provider");

    // Setup Log Appender for the log crate
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
    log::set_max_level(Level::Info.to_level_filter());
}

pub fn init_otel() -> Result<()> {
    let _ = init_logger_provider();
    let _ = init_tracer();
    init_meter_provider().with_context(|| "initialising meter provider")?;
    Ok(())
}
