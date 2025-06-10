use demtile::{elevation_from_latlon};

#[test]
fn test_elevation_from_latlon() {
    let lat = 35.97093636216379;
    let lon = 138.37020897169234;
    let zoom = 15;
    let result = elevation_from_latlon(lat, lon, zoom, None, true);
    match result {
        Ok(res) => {
            assert!(res.elevation.is_some());
            let elev = res.elevation.unwrap();
            assert!((2800.0..=2900.0).contains(&elev), "標高が想定範囲外: {}", elev);
            assert!(matches!(res.source, Some(demtile::DemSource::Dem1A) | Some(demtile::DemSource::Dem5A)));
        }
        Err(e) => panic!("エラー: {}", e),
    }
}

#[test]
fn test_elevation_from_latlon_hiroshima() {
    let lat = 34.27970691276544;
    let lon = 132.3197162299304;
    let zoom = 15;
    let result = elevation_from_latlon(lat, lon, zoom, None, true);
    match result {
        Ok(res) => {
            assert!(res.elevation.is_some());
            let elev = res.elevation.unwrap();
            assert!((500.0..=600.0).contains(&elev), "標高が想定範囲外: {}", elev);
            assert!(matches!(res.source, Some(demtile::DemSource::Dem5A)));
        }
        Err(e) => panic!("エラー: {}", e),
    }
}

#[test]
fn test_elevation_from_latlon_miyako() {
    // 岩手県宮古市付近
    let lat = 39.25;
    let lon = 141.875;
    let zoom = 15;
    let result = elevation_from_latlon(lat, lon, zoom, None, true);
    match result {
        Ok(res) => {
            assert!(res.elevation.is_some());
            let elev = res.elevation.unwrap();
            assert!((90.0..=100.0).contains(&elev), "標高が想定範囲外: {}", elev);
            assert!(matches!(res.source, Some(demtile::DemSource::Dem1A)));
        }
        Err(e) => panic!("エラー: {}", e),
    }
}

