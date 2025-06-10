//! ユーティリティ関数（ピクセル座標計算・タイルXYZ計算）

/// 緯度経度・ズーム・画像サイズからタイル内ピクセル座標を計算
pub fn latlon_to_pixel(lat: f64, lon: f64, zoom: u8, width: u32, height: u32) -> (u32, u32) {
    let z = zoom as u32;
    let n = 2f64.powi(z as i32);
    let x = (lon + 180.0) / 360.0 * n;
    let lat_rad = lat.to_radians();
    let y = (1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / std::f64::consts::PI) / 2.0 * n;
    let x_tile = x.floor() as u32;
    let y_tile = y.floor() as u32;
    let x_rel = ((x - x_tile as f64) * width as f64).floor() as u32;
    let y_rel = ((y - y_tile as f64) * height as f64).floor() as u32;
    (x_rel.min(width - 1), y_rel.min(height - 1))
}

/// 緯度経度・ズームからタイル座標(x, y, z)を計算
pub fn tile_xyz_from_latlon(lat: f64, lon: f64, zoom: u8) -> (u32, u32, u8) {
    let z = zoom as u32;
    let n = 2f64.powi(z as i32);
    let x = ((lon + 180.0) / 360.0 * n).floor() as u32;
    let lat_rad = lat.to_radians();
    let y = ((1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / std::f64::consts::PI) / 2.0 * n).floor() as u32;
    (x, y, zoom)
}
