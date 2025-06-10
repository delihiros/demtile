use demtile::elevation_from_latlon;
use std::env;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        // 標準入力CSVモード
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };
            let cols: Vec<&str> = line.trim().split(',').collect();
            if cols.len() < 2 {
                eprintln!("無効な入力: {line}");
                continue;
            }
            let lat: f64 = match cols[0].parse() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("緯度が不正: {line}");
                    continue;
                }
            };
            let lon: f64 = match cols[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("経度が不正: {line}");
                    continue;
                }
            };
            let zoom: u8 = if cols.len() > 2 {
                cols[2].parse().unwrap_or(15)
            } else {
                15
            };
            match elevation_from_latlon(lat, lon, zoom, None, true) {
                Ok(elev_result) => {
                    if let Some(elev) = elev_result.elevation {
                        println!("{:.7},{:.7},{:.2}", lat, lon, elev);
                    } else {
                        println!("{:.7},{:.7},NaN", lat, lon);
                    }
                }
                Err(e) => {
                    println!("{:.7},{:.7},ERR:{}", lat, lon, e);
                }
            }
        }
        return;
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
