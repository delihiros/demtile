//! GSI DEMタイルPNG画像を解析し標高値を取得するライブラリ

mod dem;
mod tile;
mod util;

pub use dem::{DemTile, decode_gsi_dem_pixel};
pub use tile::{dem1a_tile_url, dem5a_tile_url, fetch_and_cache_tile, fetch_dem_tile_auto};
pub use util::{latlon_to_pixel, tile_xyz_from_latlon};

use image::{ImageFormat, GenericImageView};
use std::io::Cursor;

/// 標高取得の結果と利用タイル種別
#[derive(Debug)]
pub enum DemSource {
    Dem1A,
    Dem5A,
}

pub struct ElevationResult {
    pub elevation: Option<f32>,
    pub source: Option<DemSource>,
}

/// DEM1A→DEM5Aの順で自動取得し標高データと利用タイル種別を返すAPI
pub fn elevation_from_latlon(
    lat: f64,
    lon: f64,
    zoom: u8,
    cache_db: Option<&str>,
    use_cache: bool,
) -> Result<ElevationResult, Box<dyn std::error::Error>> {
    let db_path = cache_db.unwrap_or("tile_cache.db");
    if use_cache {
        let (dem1a_url, dem5a_url) = tile::select_dem_tile_url(lat, lon, zoom);
        // DEM1A優先
        if let Ok(data) = tile::fetch_and_cache_tile(&dem1a_url, db_path) {
            if !data.is_empty() {
                let img = image::load(Cursor::new(data), ImageFormat::Png)?;
                let (width, height) = img.dimensions();
                let (x_rel, y_rel) = latlon_to_pixel(lat, lon, zoom, width, height);
                let px = img.get_pixel(x_rel, y_rel);
                if px[0] != 128 || px[1] != 0 || px[2] != 0 {
                    let elev = dem::decode_gsi_dem_pixel(px[0], px[1], px[2], px[3]);
                    return Ok(ElevationResult { elevation: elev, source: Some(DemSource::Dem1A) });
                }
            }
        }
        // DEM5A fallback
        if let Ok(data) = tile::fetch_and_cache_tile(&dem5a_url, db_path) {
            if !data.is_empty() {
                let img = image::load(Cursor::new(data), ImageFormat::Png)?;
                let (width, height) = img.dimensions();
                let (x_rel, y_rel) = latlon_to_pixel(lat, lon, zoom, width, height);
                let px = img.get_pixel(x_rel, y_rel);
                if px[0] != 128 || px[1] != 0 || px[2] != 0 {
                    let elev = dem::decode_gsi_dem_pixel(px[0], px[1], px[2], px[3]);
                    return Ok(ElevationResult { elevation: elev, source: Some(DemSource::Dem5A) });
                }
            }
        }
        Ok(ElevationResult { elevation: None, source: None })
    } else {
        Ok(ElevationResult { elevation: None, source: None })
    }
}
