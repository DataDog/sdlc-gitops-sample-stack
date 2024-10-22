use crate::coordinates::LatLong;
use crate::tiles::fetch_image_from_point;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Config;
use opentelemetry_sdk::{runtime, Resource};
use rocket::http::{ContentType, Status};
use rocket::response::{content, status};
mod coordinates;
mod tiles;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    _ = coordinates::test_fn();
    "Nothing here"
}

#[get("/health")]
fn health() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::Ok, content::RawJson("{\"status\": \"ok\"}"))
}

#[get("/images/<long>/<lat>/<size_px>")]
async fn get_image(long: f64, lat: f64, size_px: u32) -> (ContentType, Vec<u8>) {
    let image = fetch_image_from_point(LatLong(lat, long), 1.0, size_px)
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

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![health])
        .mount("/", routes![get_image])
}
