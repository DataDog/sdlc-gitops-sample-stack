use anyhow::{Context, Result};
use log::Level as log_level;
use opentelemetry::{global, trace::TracerProvider};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_resource_detectors::{OsResourceDetector, ProcessResourceDetector};
use opentelemetry_sdk::{
    logs::{Logger, LoggerProvider},
    propagation::TraceContextPropagator,
    resource::{
        EnvResourceDetector, ResourceDetector, SdkProvidedResourceDetector,
        TelemetryResourceDetector,
    },
    runtime,
    trace::Tracer,
    Resource,
};
use tracing_core::Level;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
fn init_tracer() -> Result<Tracer> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            opentelemetry_sdk::trace::Config::default().with_resource(get_resource()),
        )
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(runtime::Tokio)
        .with_context(|| "Initialising tracing pipeline")?;

    let ret = tracer_provider.tracer("tracing-otel-subscriber");
    global::set_tracer_provider(tracer_provider);
    Ok(ret)
}

// Initialize the tracing subscriber with the OpenTelemetry layer
// The OpenTelemetry layer is responsible for creating spans and propagating context
// The log layer is responsible for bridging the OpenTelemetry logs to the tracing crate
fn init_tracing_subscriber(
    log_layer: OpenTelemetryTracingBridge<LoggerProvider, Logger>,
) -> Result<()> {
    let tracer = init_tracer().with_context(|| "Initialising tracing provider")?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            Level::INFO,
        ))
        .with(tracing_subscriber::fmt::layer())
        .with(OpenTelemetryLayer::new(tracer))
        .with(log_layer)
        .init();

    Ok(())
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
// init_logger_provider returns a OpenTelemetryTracingBridge which is responsible for bridging
// the OpenTelemetry logs to the tracing crate
fn init_logger_provider() -> Result<
    OpenTelemetryTracingBridge<
        opentelemetry_sdk::logs::LoggerProvider,
        opentelemetry_sdk::logs::Logger,
    >,
> {
    log::set_max_level(log_level::Debug.to_level_filter());

    let logger_provider = opentelemetry_otlp::new_pipeline()
        .logging()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_resource(get_resource())
        .install_batch(runtime::Tokio)?;

    Ok(OpenTelemetryTracingBridge::new(&logger_provider))
}

pub fn init_otel() -> Result<()> {
    let logger_provider = init_logger_provider().with_context(|| "initialising logger_provider")?;
    init_tracing_subscriber(logger_provider).with_context(|| "Initialising tracing subscriber")?;
    init_meter_provider().with_context(|| "initialising meter provider")?;
    Ok(())
}
