use std::collections::HashMap;

use crate::coordinates::LatLong;
use crate::tiles::fetch_image_from_point;
use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use log::{info, warn};
use opentelemetry_instrumentation_actix_web::RequestTracing;
use tiles::TileSet;
mod coordinates;
mod tiles;

mod telemetry_conf;
use telemetry_conf::init_otel;

async fn index() -> impl Responder {
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

    info!(
        latitude = lat,
        longitude = long;
        "Fetching image"
    );

    match fetch_image_from_point(LatLong(lat, long), radius, size_px, tileset).await {
        Ok(image) => HttpResponse::Ok()
            .content_type(ContentType::png())
            .body(image),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Roll otel errors up to here and log them in aggregate
    // Hold providers to keep them alive until shutdown
    let otel_providers = match init_otel() {
        Ok(providers) => {
            info!("Successfully configured OTel");
            Some(providers)
        }
        Err(err) => {
            warn!(
                "Couldn't start OTel! Will proudly soldier on without telemetry: {0}",
                err
            );
            None
        }
    };

    let server_result = HttpServer::new(|| {
        App::new()
            .wrap(RequestTracing::new())
            .route("/", web::get().to(index))
            .route("/ping", web::get().to(health))
            .service(get_image)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;

    // Explicitly shutdown OpenTelemetry providers before exiting
    if let Some((tracer_provider, meter_provider, logger_provider)) = otel_providers {
        info!("Shutting down OpenTelemetry providers");

        if let Err(err) = tracer_provider.shutdown() {
            warn!("Error shutting down tracer provider: {:?}", err);
        }

        if let Err(err) = meter_provider.shutdown() {
            warn!("Error shutting down meter provider: {:?}", err);
        }

        if let Some(logger_provider) = logger_provider {
            if let Err(err) = logger_provider.shutdown() {
                warn!("Error shutting down logger provider: {:?}", err);
            }
        }
    }

    server_result
}
