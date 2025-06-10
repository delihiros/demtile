use demtile::tile_xyz_from_latlon;

#[test]
fn test_tile_xyz_from_latlon_tokyo_skytree() {
    // 東京スカイツリー付近
    let lat = 35.709529;
    let lon = 139.810713;
    let zoom = 15;
    let (x, y, z) = tile_xyz_from_latlon(lat, lon, zoom);
    assert_eq!(z, 15);
    assert_eq!(x, 29109, "x tile mismatch");
    assert_eq!(y, 12900, "y tile mismatch");
}

#[test]
fn test_tile_xyz_from_latlon_hiroshima() {
    // 宮島付近
    let lat = 34.278602;
    let lon = 132.317662;
    let zoom = 15;
    let (x, y, z) = tile_xyz_from_latlon(lat, lon, zoom);
    assert_eq!(z, 15);
    assert_eq!(x, 28427, "x tile mismatch");
    assert_eq!(y, 13059, "y tile mismatch");
}
