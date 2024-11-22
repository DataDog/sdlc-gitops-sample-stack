use std::collections::HashMap;

use crate::coordinates::LatLong;
use crate::tiles::fetch_image_from_point;
use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use opentelemetry::trace::{Tracer, TracerProvider as _};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::LogExporter;
use opentelemetry_sdk::logs::{BatchLogProcessor, LoggerProvider};
use opentelemetry_sdk::runtime;
use opentelemetry_sdk::trace::TracerProvider;
use tiles::TileSet;
use tracing_actix_web::TracingLogger;
mod coordinates;
mod tiles;

use tracing::{error, event, info, span, Level};
use tracing_subscriber::Registry;
use tracing_subscriber::{layer::SubscriberExt, Layer};

mod telemetry_conf;
use telemetry_conf::init_otel;
use tracing_subscriber::{fmt::format::FmtSpan, util::SubscriberInitExt};

async fn index() -> impl Responder {
    info!("Hit index, but there's nothing there!");
    "Nothing here"
}

async fn health() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body("{\"status\": \"ok\"}")
}

#[get("/images/{long}/{lat}/{size_px}")]
async fn get_image(
    path: web::Path<(f64, f64, u32)>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let (long, lat, size_px) = path.into_inner();

    // Extract optional parameters from the query map
    let radius = query
        .get("radius")
        .and_then(|r| r.parse().ok())
        .unwrap_or(1.0);
    let tileset = query
        .get("tileset")
        .map(|t| match t.as_str() {
            "swisstopo" => TileSet::Swisstopo,
            _ => TileSet::Osm,
        })
        .unwrap_or(TileSet::Osm);

    info!(latitude = lat, longitude = long);

    let image = fetch_image_from_point(LatLong(lat, long), radius, size_px, tileset)
        .await
        .expect("It should work")
        .to_vec();

    HttpResponse::Ok()
        .content_type(ContentType::png())
        .body(image)
}

enum InstrumentationMode {
    TracingStdOut,
    OtelStdOut,
    OtelOtlpMode,
}

const TRACING_MODE: InstrumentationMode = InstrumentationMode::OtelOtlpMode;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match TRACING_MODE {
        InstrumentationMode::TracingStdOut => {
            println!("TracingStdOut mode");
            tracing_subscriber::fmt()
                .with_span_events(FmtSpan::ENTER)
                .init();
        }
        InstrumentationMode::OtelStdOut => {
            println!("OtelStdOut mode");
            // Create the OTEL STD Out provider
            let exporter = opentelemetry_stdout::SpanExporter::default();
            let provider = TracerProvider::builder()
                .with_simple_exporter(exporter)
                .build();
            let tracer = provider.tracer("pass-image-api");

            // Create our OTEL tracing subscriber and subscribe it
            let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
            let _subscriber = Registry::default().with(telemetry).init();
        }
        InstrumentationMode::OtelOtlpMode => {
            println!("OtelOtlp Mode");

            //
            // First, tracing
            //
            let exporter = opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .build()
                .expect("I can build an exporter");

            let provider = TracerProvider::builder()
                .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
                .build();
            let tracer = provider.tracer("pass-image-api");

            // Create our OTEL tracing subscriber, and subscribe it
            let otel_layer = tracing_opentelemetry::layer()
                .with_tracer(tracer)
                .with_filter(tracing_subscriber::filter::LevelFilter::from_level(
                    tracing_core::Level::INFO,
                ));

            //
            // Now logging
            //
            let log_exporter = LogExporter::builder()
                .with_tonic()
                .build()
                .expect("I can make a log exporter");

            let logger_provider = LoggerProvider::builder()
                .with_log_processor(
                    BatchLogProcessor::builder(log_exporter, runtime::Tokio).build(),
                )
                .build();
            let otel_tracing_bridge = OpenTelemetryTracingBridge::new(&logger_provider)
                .with_filter(tracing_subscriber::filter::LevelFilter::from_level(
                    tracing_core::Level::INFO,
                ));
            tracing_subscriber::registry()
                .with(otel_layer)
                .with(otel_tracing_bridge)
                .init();

            info!(name: "my-event-name", target: "started-up", event_id = 20, user_name = "otel", user_email = "otel@opentelemetry.io", message = "This is an example message");

            println!("Started telemetry config");
        }
    }

    println!("Starting things up");

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(index))
            .route("/ping", web::get().to(health))
            .service(get_image)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
