//! DEMタイルの構造体と標高データのデコード処理

use image::{GenericImageView, ImageReader};
use std::path::Path;

/// DEMタイルPNGから標高値を取得する構造体
pub struct DemTile {
    pub width: u32,
    pub height: u32,
    pub elevations: Vec<Option<f32>>, // Noneは欠損値
}

impl DemTile {
    /// PNGファイルからDemTileを生成
    pub fn from_png_file<P: AsRef<Path>>(path: P) -> image::ImageResult<Self> {
        let img = ImageReader::open(path)?.decode()?;
        Self::from_image(img)
    }

    /// image::DynamicImageからDemTileを生成
    pub fn from_image(img: image::DynamicImage) -> image::ImageResult<Self> {
        let (width, height) = img.dimensions();
        let mut elevations = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                let px = img.get_pixel(x, y);
                let (r, g, b, a) = (px[0], px[1], px[2], px[3]);
                let elev = decode_gsi_dem_pixel(r, g, b, a);
                elevations.push(elev);
            }
        }
        Ok(DemTile { width, height, elevations })
    }

    /// 指定座標の標高値を取得
    pub fn get_elevation(&self, x: u32, y: u32) -> Option<f32> {
        if x < self.width && y < self.height {
            self.elevations[(y * self.width + x) as usize]
        } else {
            None
        }
    }
}

/// GSI DEMタイルPNGのピクセル値から標高値(m)をデコード（公式仕様準拠）
pub fn decode_gsi_dem_pixel(r: u8, g: u8, b: u8, _a: u8) -> Option<f32> {
    // 無効値 (128,0,0)
    if r == 128 && g == 0 && b == 0 {
        return None;
    }
    let x = 256 * 256 * (r as u32) + 256 * (g as u32) + (b as u32);
    let u = 0.01_f32;
    if x < 223 {
        Some(x as f32 * u)
    } else if x == 223 {
        None // NA
    } else {
        Some(((x - 224) as f32) * u)
    }
}
