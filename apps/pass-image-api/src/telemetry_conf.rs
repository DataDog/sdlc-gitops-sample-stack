use log::Level as log_level;
use opentelemetry::{global, logs::LogError, trace::TracerProvider};
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

fn init_tracer() -> Tracer {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            opentelemetry_sdk::trace::Config::default().with_resource(get_resource()),
        )
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(runtime::Tokio)
        .unwrap();

    global::set_tracer_provider(tracer_provider.clone());
    
    tracer_provider.tracer("tracing-otel-subscriber")
}

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

fn init_meter_provider() -> () {
    let meter_provider = opentelemetry_otlp::new_pipeline()
        .metrics(opentelemetry_sdk::runtime::Tokio)
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_resource(get_resource())
        .with_delta_temporality()
        .build();

    global::set_meter_provider(meter_provider.expect("Failed to initialize meter provider"));
}

fn init_logger_provider() -> Result<opentelemetry_sdk::logs::LoggerProvider, LogError> {
    log::set_max_level(log_level::Debug.to_level_filter());

    Ok(opentelemetry_otlp::new_pipeline()
        .logging()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_resource(get_resource())
        .install_batch(runtime::Tokio).unwrap())    
}

pub fn init_otel() -> () {
    let logger_provider = init_logger_provider().unwrap();
    let log_layer = OpenTelemetryTracingBridge::new(&logger_provider);

    init_tracing_subscriber(log_layer);
    init_meter_provider();
}