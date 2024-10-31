# Pass Image API
This rust service provides an aerial image of a given location using openstreetmaps tiles. The caller simply provides:

* latitude and longitude in decimal-degrees
* the radius in kilometers around the point to cover
* the output size in pixels - we generate square images, so this is both width and height

... and a PNG will be returned.

# Usage

```bash
# Point the OTEL_EXPORTER_OTLP_ENDPOINT to either your OTel Collector endpoint or your Datadog Agent
# Start the service
OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317 OTEL_SERVICE_NAME=pass-image-api cargo run &

#
# URL format is /images/<long>/<lat>/<size_in_px>
# An optional ?radius=x.y can be provided to specify the radius in kilometers about the point
# The default radius is 1.0km
# An optional ?tileset=... can be added to specify the tileset.
# The default is osm, 'swisstopo' is also supported for points in Switzerland

# Get an 512x512 image centered over Perth, Western Australia
curl "http://localhost:8080/images/115.85870047525302/-31.95271807274208/512" -o perth.png

# Get a 1024x1024 image centered over the Grosse Scheidegg pass, Switzerland. 
curl "http://localhost:8080/images/8.102121/46.655559/1024?radius=3.0" -o grosse-scheidegg.png

```

**Perth, WA**:
http://localhost:8000/images/115.85870047525302/-31.95271807274208/512

**Grosse Scheidegg*:
