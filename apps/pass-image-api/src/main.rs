use crate::coordinates::LatLong;
use crate::tiles::fetch_image_from_point;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Config;
use opentelemetry_sdk::{runtime, Resource};
use rocket::http::{ContentType, Status};
use rocket::response::{content, status};
use tiles::TileSet;
mod coordinates;
mod tiles;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Nothing here"
}

#[get("/ping")]
fn health() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::Ok, content::RawJson("{\"status\": \"ok\"}"))
}

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
    // An example from here --> https://github.com/open-telemetry/opentelemetry-rust/blob/main/opentelemetry-otlp/examples/basic-otlp/src/main.rs
    // or from here --> https://github.com/open-telemetry/opentelemetry-rust/blob/main/opentelemetry-otlp/examples/basic-otlp-http/src/main.rs

    let resource = Resource::new(vec![KeyValue::new(
        opentelemetry_semantic_conventions::resource::SERVICE_NAME,
        "pass-image-api",
    )]);

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://localhost:1234");

    let _tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(Config::default().with_resource(resource))
        .install_batch(runtime::Tokio)
        .unwrap();

    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", 8080));

    rocket::custom(figment)
        .mount("/", routes![index])
        .mount("/", routes![health])
        .mount("/", routes![get_image])
}
