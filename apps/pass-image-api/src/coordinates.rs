// ! # coordinates
// !
// ! The coordinates module provides types and utilities for dealing with geospatial
// ! coordinates. For our purposes this means converting between latitude/longitude WGS84
// ! pairs and webmercator slippy-maps style tile coordinates.
// !

use log::debug;

// A latitude/longitude pair
#[derive(Debug, Clone, Copy)]
pub struct LatLong(pub f64, pub f64);

// A tile coordinate. Note that a 'zoomLevel' value
// must be carried along with this too
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TileCoordinate {
    pub x: f32,
    pub y: f32,
    pub z: u32,
}

// Converts a lat/long pair to tile coordinates at a particular zoom
pub fn lat_long_to_tile_coords(point: &LatLong, zoom: u32) -> TileCoordinate {
    let lat_rad = point.0.to_radians();
    let n = 2.0_f64.powi(zoom as i32);
    let x_tile = (point.1 + 180.0) / 360.0 * n;
    let y_tile =
        (1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / std::f64::consts::PI) / 2.0 * n;

    debug!("Center coord at z={2}: {0}, {1}", x_tile, y_tile, zoom);
    TileCoordinate {
        x: x_tile as f32,
        y: y_tile as f32,
        z: zoom,
    }
}

// An extension of a TileBox that allows us to specify extra information to constrain it. The inner_size
// is the number of pixels that are actually "used", and the center is the center the TileBox was taken around.
// This is a bit of a funny type as it mixes coordinate systems; it would be better if we changed this so that
// we have pixel offsets into the image here based on centering around the point we have created the ConstrainedTileBox
// for.
#[derive(Debug, Copy, Clone)]
pub struct ConstrainedTileBox {
    pub center: LatLong,
    pub tile_box: TileBox,
    pub inner_size_px: (u32, u32),
}

// A box of tiles
#[derive(Debug, Copy, Clone)]
pub struct TileBox {
    pub top_left: TileCoordinate,
    pub bottom_right: TileCoordinate,
}

impl TileBox {
    pub fn _outer_size_px(&self) -> (u32, u32) {
        (
            (256.0 * (self.bottom_right.x - self.top_left.x)) as u32,
            (256.0 * (self.bottom_right.y - self.top_left.y)) as u32,
        )
    }

    pub fn outer_top_left(&self) -> (u32, u32) {
        (
            self.top_left.x.floor() as u32,
            self.top_left.y.floor() as u32,
        )
    }
}

// Given a point on the earth, a radius, and a desired zoom level, this function produces a
// ConstrainedTileBox that contains enough pixels to cover the given area.
fn lat_long_and_radius_to_tile_box(
    point: &LatLong,
    radius_km: f32,
    zoom: u32,
) -> ConstrainedTileBox {
    let earth_radius_km = 6371.0;

    // Convert the center point to tile coordinates
    let center_tile = lat_long_to_tile_coords(point, zoom);

    // Calculate the approximate size of one tile in kilometers at the given zoom level
    let tile_size_km = tile_size_kms(zoom, earth_radius_km);

    // Calculate the number of tiles that fit into the radius (in both directions)
    let radius_tiles = radius_km / tile_size_km;

    // Create a square bounding box by using the same radius in both x and y directions
    let top_left_tile = TileCoordinate {
        x: center_tile.x - radius_tiles,
        y: center_tile.y - radius_tiles,
        z: zoom,
    };
    let bottom_right_tile = TileCoordinate {
        x: center_tile.x + radius_tiles,
        y: center_tile.y + radius_tiles,
        z: zoom,
    };

    // What's the inner resolution for our given radius? E.g., if we get zoom level '0' and ask
    // for a 10k radius, it's going to be very close to zero pixels
    let inner_size_px = (256.0 * radius_tiles) as u32;

    // Print some helpful debugging info
    debug!(
        "At zoom {0}, one tile has edge {1:.2} km. That means we need {2:.2} tiles for our radius. Our inner size is {3:.2} pixels",
        zoom, tile_size_km, radius_tiles, inner_size_px
    );

    ConstrainedTileBox {
        center: *point,
        inner_size_px: (inner_size_px, inner_size_px),
        tile_box: TileBox {
            top_left: top_left_tile,
            bottom_right: bottom_right_tile,
        },
    }
}

// tile_size_kms calculates the size of a tile at the given zoom level in kilometers.
// in webmercator, the size of a tile is the same on both axes
fn tile_size_kms(zoom: u32, earth_radius_km: f64) -> f32 {
    let n = 2.0_f64.powi(zoom as i32);
    ((earth_radius_km * 2.0 * std::f64::consts::PI) / n) as f32
}

// Given a center point, a desired image size, and a radius in kilometers, produces
// a ConstrainedTileBox that provides enough pixels to cover the given area, ensuring
// we have (image_size_px / 2) pixels available to the left/right/above/below of the
// center point. This also means we have to pick an appropriate zoom level to get
// the resolution we need.
pub fn lat_long_and_image_size_to_bounding_box(
    center: LatLong,
    radius_km: f32,
    image_size_px: u32,
) -> ConstrainedTileBox {
    // Generate a list of zoom levels from 0 to 21
    let zooms: Vec<u32> = (0..=21).collect();
    let candidates: Vec<(u32, ConstrainedTileBox)> = zooms
        .iter()
        .map(|z| (*z, lat_long_and_radius_to_tile_box(&center, radius_km, *z)))
        .filter(|c| c.1.inner_size_px.0 > image_size_px)
        .take(1) // stop once we find one
        .collect();

    let best_candidate = candidates.first().expect("We shoulda found one");
    debug!(
        "best_candidate: {0}, {1:?}",
        best_candidate.0, best_candidate.1
    );

    best_candidate.1
}

#[cfg(test)]
mod tests {

    use super::*;
    use float_cmp::*;

    const MARGIN: F32Margin = F32Margin {
        ulps: 2,
        epsilon: 0.0,
    };

    #[test]
    fn test_zero() {
        let TileCoordinate { x, y, .. } = lat_long_to_tile_coords(&LatLong(-31.0, 115.0), 0);

        debug!("{0}, {1}", x, y);

        assert!(x.approx_eq(0.819_444_4, MARGIN));
        assert!(y.approx_eq(0.590_648_7, MARGIN));
    }

    #[test]
    fn test_lat_long_to_tile_coords_perth() {
        let lat = -31.9514;
        let lon = 115.8617;
        let zoom = 12;
        let TileCoordinate { x, y, z } = lat_long_to_tile_coords(&LatLong(lat, lon), zoom);

        assert!(x.approx_eq(3_366.248_8, MARGIN));
        assert!(y.approx_eq(2_431.989_7, MARGIN));
        assert_eq!(z, zoom);
    }

    #[test]
    fn test_lat_long_to_tile_coords_thun() {
        let lat = 46.7580;
        let lon = 7.6280;
        let zoom = 14;
        let TileCoordinate { x, y, z } = lat_long_to_tile_coords(&LatLong(lat, lon), zoom);
        assert!(x.approx_eq(8539.159, MARGIN));
        assert!(y.approx_eq(5778.795, MARGIN));
        assert_eq!(z, zoom);
    }

    #[test]
    fn test_lat_long_and_radius_to_tile_box_perth() {
        let lat = -31.9514;
        let lon = 115.8617;
        let zoom = 12;
        let radius_km = 2.0;

        // Convert the latitude and longitude of Perth to tile coordinates and calculate the bounding box
        let ConstrainedTileBox {
            tile_box:
                TileBox {
                    top_left,
                    bottom_right,
                },
            ..
        } = lat_long_and_radius_to_tile_box(&LatLong(lat, lon), radius_km, zoom);

        // Assertions - rough values, need fixing with exact ones
        let TileCoordinate {
            x: top_left_x,
            y: top_left_y,
            z: _top_left_z,
        } = top_left;
        let TileCoordinate {
            x: bottom_right_x,
            y: bottom_right_y,
            z: _bottom_right_z,
        } = bottom_right;

        assert!(top_left_x.approx_eq(3_366.044_2, MARGIN));
        assert!(top_left_y.approx_eq(2_431.785_2, MARGIN));
        assert!(bottom_right_x.approx_eq(3_366.453_4, MARGIN));
        assert!(bottom_right_y.approx_eq(2_432.194_3, MARGIN));
    }

    #[test]
    fn test_lat_long_and_image_size_to_bounding_box_perth() {
        let lat = -31.9514;
        let lon = 115.8617;
        let radius_km = 10.0;
        let image_size_px = 1000;

        // Call the function to get the best zoom level and bounding box
        let ConstrainedTileBox {
            tile_box:
                TileBox {
                    top_left,
                    bottom_right,
                },
            ..
        } = lat_long_and_image_size_to_bounding_box(LatLong(lat, lon), radius_km, image_size_px);

        // Rough assertions for the zoom and tile coordinates
        // assert_eq!(zoom, 14); // Adjust this value based on actual results

        let TileCoordinate {
            x: top_left_x,
            y: top_left_y,
            z: _top_left_z,
        } = top_left;
        let TileCoordinate {
            x: bottom_right_x,
            y: bottom_right_y,
            z: _bottom_right_z,
        } = bottom_right;

        assert!(top_left_x.approx_eq(13_460.902, MARGIN));
        assert!(top_left_y.approx_eq(9_723.866, MARGIN));
        assert!(bottom_right_x.approx_eq(13_469.088, MARGIN));
        assert!(bottom_right_y.approx_eq(9_732.052, MARGIN));
    }
}
