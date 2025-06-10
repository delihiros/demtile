use demtile::dem1a_tile_url;

#[test]
fn test_dem1a_tile_url_miyako_status() {
    // 岩手県宮古市付近
    let lat = 39.25;
    let lon = 141.875;
    let zoom = 15;
    let url = dem1a_tile_url(lat, lon, zoom);
    let resp = reqwest::blocking::get(&url).expect("HTTPリクエスト失敗");
    assert!(resp.status().is_success(), "DEM1A URLが200 OKではありません: status={}", resp.status());
}
