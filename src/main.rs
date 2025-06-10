use demtile::elevation_from_latlon;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <lat> <lon> [zoom]", args[0]);
        std::process::exit(1);
    }
    let lat: f64 = args[1].parse().expect("Invalid latitude");
    let lon: f64 = args[2].parse().expect("Invalid longitude");
    let zoom: u8 = if args.len() > 3 {
        args[3].parse().expect("Invalid zoom")
    } else {
        15
    };
    match elevation_from_latlon(lat, lon, zoom, None, true) {
        Ok(elev_result) => {
            if let Some(elev) = elev_result.elevation {
                println!("標高: {:.2}m", elev);
            } else {
                println!("標高データが取得できませんでした");
            }
        }
        Err(e) => eprintln!("エラー: {}", e),
    }
}
