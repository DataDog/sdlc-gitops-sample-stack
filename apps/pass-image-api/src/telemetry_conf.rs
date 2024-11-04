use log::Level as log_level;
use opentelemetry::{global, trace::TracerProvider};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp;
use opentelemetry_sdk::{
    logs::{
        LoggerProvider, Logger
    },
    propagation::TraceContextPropagator,
    resource::{
        EnvResourceDetector, ResourceDetector, SdkProvidedResourceDetector, TelemetryResourceDetector,
    },
    runtime, Resource,
    trace::Tracer
};
use opentelemetry_resource_detectors::{
    OsResourceDetector, ProcessResourceDetector
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
fn init_tracer() -> Tracer {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            opentelemetry_sdk::trace::Config::default().with_resource(get_resource()),
        )
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(runtime::Tokio)
        .expect("Failed to initialize tracer provider");

    global::set_tracer_provider(tracer_provider.clone());
    
    tracer_provider.tracer("tracing-otel-subscriber")
}

// Initialize the tracing subscriber with the OpenTelemetry layer
// The OpenTelemetry layer is responsible for creating spans and propagating context
// The log layer is responsible for bridging the OpenTelemetry logs to the tracing crate
fn init_tracing_subscriber(log_layer: OpenTelemetryTracingBridge<LoggerProvider, Logger>) -> () {
    let tracer = init_tracer();

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            Level::INFO,
        ))
        .with(tracing_subscriber::fmt::layer())
        .with(OpenTelemetryLayer::new(tracer))
        .with(log_layer)
        .init();
}

// A Meter Provider is a factory for Meters
// A Meter creates metric instruments, capturing measurements about a service at runtime.
fn init_meter_provider() -> () {
    let meter_provider = opentelemetry_otlp::new_pipeline()
        .metrics(opentelemetry_sdk::runtime::Tokio)
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_resource(get_resource())
        .with_delta_temporality()
        .build();

    global::set_meter_provider(meter_provider.expect("Failed to initialize meter provider"));
}

// A Logger Provider is a factory for Loggers
// init_logger_provider returns a OpenTelemetryTracingBridge which is responsible for bridging
// the OpenTelemetry logs to the tracing crate
fn init_logger_provider() -> OpenTelemetryTracingBridge<opentelemetry_sdk::logs::LoggerProvider, opentelemetry_sdk::logs::Logger> {
    log::set_max_level(log_level::Debug.to_level_filter());

    let logger_provider = opentelemetry_otlp::new_pipeline()
        .logging()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_resource(get_resource())
        .install_batch(runtime::Tokio)
        .expect("Failed to initialize logger provider");

    OpenTelemetryTracingBridge::new(&logger_provider)
}

pub fn init_otel() -> () {
    init_tracing_subscriber(init_logger_provider());
    init_meter_provider();
}