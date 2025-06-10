//! タイルのダウンロード・キャッシュ・URL生成

use reqwest::blocking::get;
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::PathBuf;

/// 指定した緯度経度・ズームレベルからDEM5AタイルのURLを生成
pub fn dem5a_tile_url(lat: f64, lon: f64, zoom: u8) -> String {
    let (x, y, z) = crate::util::tile_xyz_from_latlon(lat, lon, zoom);
    format!("https://cyberjapandata.gsi.go.jp/xyz/dem5a_png/{}/{}/{}.png", z, x, y)
}

/// 指定した緯度経度・ズームレベルからDEM1AタイルのURLを生成
pub fn dem1a_tile_url(lat: f64, lon: f64, zoom: u8) -> String {
    let (x, y, z) = crate::util::tile_xyz_from_latlon(lat, lon, zoom);
    format!("https://cyberjapandata.gsi.go.jp/xyz/dem1a_png/{}/{}/{}.png", z, x, y)
}

/// PNGタイルをキャッシュしつつ取得する関数
/// PNGはファイルシステムに保存し、sqlite3にはパスのみを保存
pub fn fetch_and_cache_tile(url: &str, cache_db: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let cache_dir = "tile_cache";
    fs::create_dir_all(cache_dir)?;

    // DB接続
    let conn = Connection::open(cache_db)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tile_cache (
            url TEXT PRIMARY KEY,
            path TEXT,
            updated_at INTEGER
        )",
        [],
    )?;
    // キャッシュ検索
    if let Ok(mut stmt) = conn.prepare("SELECT path FROM tile_cache WHERE url = ?1") {
        let mut rows = stmt.query(params![url])?;
        if let Some(row) = rows.next()? {
            let path: String = row.get(0)?;
            let mut file = fs::File::open(&path)?;
            let mut data = Vec::new();
            file.read_to_end(&mut data)?;
            return Ok(data);
        }
    }
    // なければダウンロード
    let resp = get(url)?;
    if !resp.status().is_success() {
        return Err(format!("HTTP error: {}", resp.status()).into());
    }
    let bytes = resp.bytes()?;
    let data = bytes.to_vec();
    // PNGかどうか簡易チェック（先頭8バイト）
    if data.len() < 8 || &data[0..8] != b"\x89PNG\r\n\x1a\n" {
        return Err("取得データがPNGではありません".into());
    }
    // ファイル名はURLのSHA256
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    let mut path = PathBuf::from(cache_dir);
    path.push(format!("{}.png", hash));
    fs::write(&path, &data)?;
    // キャッシュ保存
    conn.execute(
        "INSERT OR REPLACE INTO tile_cache (url, path, updated_at) VALUES (?1, ?2, strftime('%s','now'))",
        params![url, path.to_string_lossy()],
    )?;
    Ok(data)
}

/// 指定した緯度経度・ズームレベルからDEM1A/DEM5AタイルのURLを自動選択して返す
pub fn select_dem_tile_url(lat: f64, lon: f64, zoom: u8) -> (String, String) {
    let dem1a_url = dem1a_tile_url(lat, lon, zoom);
    let dem5a_url = dem5a_tile_url(lat, lon, zoom);
    (dem1a_url, dem5a_url)
}

/// DEM1A→DEM5Aの順で取得し、どちらもなければNoneを返す
pub fn fetch_dem_tile_auto(lat: f64, lon: f64, zoom: u8, cache_db: &str) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
    let (dem1a_url, dem5a_url) = select_dem_tile_url(lat, lon, zoom);
    // DEM1A優先
    match fetch_and_cache_tile(&dem1a_url, cache_db) {
        Ok(data) => {
            // 画像が有効かチェック（無効画像は128,0,0が多い）
            if !data.is_empty() && !is_invalid_dem_png(&data) {
                return Ok(Some(data));
            }
        }
        Err(_) => {}
    }
    // DEM5A fallback
    match fetch_and_cache_tile(&dem5a_url, cache_db) {
        Ok(data) => {
            if !data.is_empty() && !is_invalid_dem_png(&data) {
                return Ok(Some(data));
            }
        }
        Err(_) => {}
    }
    Ok(None)
}

/// DEM PNGが無効値画像か判定（先頭ピクセルが128,0,0なら無効とみなす）
fn is_invalid_dem_png(data: &[u8]) -> bool {
    if let Ok(img) = image::load_from_memory(data) {
        let rgb = img.to_rgb8();
        let px = rgb.get_pixel(0, 0);
        px[0] == 128 && px[1] == 0 && px[2] == 0
    } else {
        true
    }
}
