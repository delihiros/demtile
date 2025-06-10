# demtile

国土地理院のDEMタイル（PNG形式）から標高値を取得するRustライブラリです。

## 特徴
- GSI（国土地理院）DEM1A/DEM5AタイルのPNG画像から標高値をデコード
- 緯度経度から自動でタイルをダウンロードし標高を取得
- SQLite+ファイルキャッシュ対応
- CLI/ライブラリ両対応

## 使い方

```rust
use demtile::elevation_from_latlon;
let result = elevation_from_latlon(35.0, 135.0, 15, None, true)?;
println!("標高: {:?}m", result.elevation);
```

## CLI

```
cargo run -- <lat> <lon> [zoom]
```

## ライセンス
MIT
