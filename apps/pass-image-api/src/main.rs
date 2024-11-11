use crate::coordinates::LatLong;
use crate::tiles::fetch_image_from_point;
use rocket::http::{ContentType, Status};
use rocket::response::{content, status};
use tiles::TileSet;
mod coordinates;
mod tiles;

use tracing::{info, warn};

mod telemetry_conf;
use telemetry_conf::init_otel;

#[macro_use]
extern crate rocket;

#[tracing::instrument(name = "GET /", fields(otel.kind = "server"), skip_all)]
#[get("/")]
fn index() -> &'static str {
    "Nothing here"
}

#[get("/ping")]
fn health() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::Ok, content::RawJson("{\"status\": \"ok\"}"))
}

#[tracing::instrument(
    name = "GET /images/{long}/{lat}/{size_px}?{radius}&{tileset}",
    fields(otel.kind = "server"),
    skip_all
)]
#[get("/images/<long>/<lat>/<size_px>?<radius>&<tileset>")]
async fn get_image(
    long: f64,
    lat: f64,
    size_px: u32,
    radius: Option<f32>,
    tileset: Option<String>,
) -> (ContentType, Vec<u8>) {
    let target_tileset = match tileset {
        Some(v) => match v.as_str() {
            "swisstopo" => TileSet::Swisstopo,
            _ => TileSet::Osm,
        },
        None => TileSet::Osm,
    };

    info!("Fetching image for lat: {}, long: {}, size: {}", lat, long, size_px);
    let radius = radius.unwrap_or(1.0);
    let image = fetch_image_from_point(LatLong(lat, long), radius, size_px, target_tileset)
        .await
        .expect("It should work")
        .to_vec();

    (ContentType::PNG, image)
}

// We make this `async`, because if we don't, rocket hasn't started
// tokio yet, and rocket isn't happy it can't find it
#[launch]
async fn rocket() -> _ {
    // Roll otel errors up to here and log them in aggregate
    match init_otel() {
        Ok(_) => {
            info!("Successfully configured OTel");
        }
        Err(err) => {
            warn!(
                "Couldn't start otel! Will proudly soldier on without telemetry: {0}",
                err
            );
        }
    };

    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 8080));

    rocket::custom(figment)
        .mount("/", routes![index])
        .mount("/", routes![health])
        .mount("/", routes![get_image])
}
