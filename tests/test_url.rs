use demtile::dem1a_tile_url;

#[test]
fn test_dem1a_tile_url_hiroshima() {
    // 広島市付近
    let lat = 34.278602;
    let lon = 132.317662;
    let zoom = 15;
    let url = dem1a_tile_url(lat, lon, zoom);
    // 期待されるタイル座標: 15/28427/13059
    assert!(url.ends_with("/15/28427/13059.png"), "URLが想定と異なります: {}", url);
}
