use std::collections::HashMap;

use crate::coordinates::LatLong;
use crate::tiles::fetch_image_from_point;
use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use actix_web_opentelemetry::RequestTracing;
use log::{info, warn};
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
        "Fetching image for lat: {}, long: {}, size: {}",
        lat, long, size_px
    );
    let image = fetch_image_from_point(LatLong(lat, long), radius, size_px, tileset)
        .await
        .expect("It should work")
        .to_vec();

    HttpResponse::Ok()
        .content_type(ContentType::png())
        .body(image)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Roll otel errors up to here and log them in aggregate
    match init_otel() {
        Ok(_) => {
            info!("Successfully configured OTel");
        }
        Err(err) => {
            warn!(
                "Couldn't start OTel! Will proudly soldier on without telemetry: {0}",
                err
            );
        }
    };

    HttpServer::new(|| {
        App::new()
            .wrap(RequestTracing::new())
            .route("/", web::get().to(index))
            .route("/ping", web::get().to(health))
            .service(get_image)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
