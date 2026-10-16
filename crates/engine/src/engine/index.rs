pub use domain::types::gis_engine_type::{
    MapEngineInput, MapObject, MapXYZ, MoveXY, Screen, TileZXY,
};
use ogc_tms::{BoundingBox, TileMatrixSet};
use proj_core::Transform;

/// 投影XY座標を地理座標へ変換する。
pub fn xy_to_coordinate(
    transform: &Transform,
    xy: (f64, f64),
) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    Ok(transform.convert(xy)?)
}

/// 地理座標を投影XY座標へ変換する。
pub fn coordinate_to_xy(
    transform: &Transform,
    coordinate: (f64, f64),
) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    Ok(transform.convert(coordinate)?)
}

/// 左上原点の画面ピクセル座標を経度・緯度へ変換する。
///
/// 戻り値は (経度, 緯度)。ピクセルXは右向き、Yは下向き。
pub fn pixel_xy_to_lon_lat(
    pixel_xy: (f64, f64),
    map_xyz: &MapXYZ,
    screen: Screen,
) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    if !pixel_xy.0.is_finite() || !pixel_xy.1.is_finite() {
        return Err("ピクセル座標が無効です".into());
    }

    let (center_xy, resolution, _forward, inverse) = pixel_conversion_context(map_xyz, screen)?;
    let offset_x = pixel_xy.0 - screen.width / 2.0;
    let offset_y = pixel_xy.1 - screen.height / 2.0;
    let (longitude, latitude) = xy_to_coordinate(
        &inverse,
        (
            center_xy.0 + offset_x * resolution,
            center_xy.1 - offset_y * resolution,
        ),
    )?;

    Ok((longitude, latitude))
}

/// 経度・緯度を左上原点の画面ピクセル座標へ変換する。
pub fn lon_lat_to_pixel_xy(
    longitude: f64,
    latitude: f64,
    map_xyz: &MapXYZ,
    screen: Screen,
) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    if !latitude.is_finite()
        || !(-90.0..=90.0).contains(&latitude)
        || !longitude.is_finite()
        || !(-180.0..=180.0).contains(&longitude)
    {
        return Err("緯度または経度が無効です".into());
    }

    let (center_xy, resolution, forward, _inverse) = pixel_conversion_context(map_xyz, screen)?;
    let latitude = latitude.clamp(-ogc_tms::MERCATOR_LAT_LIMIT, ogc_tms::MERCATOR_LAT_LIMIT);
    let coordinate_xy = coordinate_to_xy(&forward, (longitude, latitude))?;

    Ok((
        screen.width / 2.0 + (coordinate_xy.0 - center_xy.0) / resolution,
        screen.height / 2.0 + (center_xy.1 - coordinate_xy.1) / resolution,
    ))
}

fn pixel_conversion_context(
    map_xyz: &MapXYZ,
    screen: Screen,
) -> Result<((f64, f64), f64, Transform, Transform), Box<dyn std::error::Error>> {
    if !validate_map_xyz(map_xyz) {
        return Err("地図の中心座標またはズームレベルが無効です".into());
    }
    if !validate_screen(&screen) {
        return Err("地図画面のサイズが無効です".into());
    }

    let zoom = map_xyz.zoom as u8;
    let tile_matrix_set = ogc_tms::web_mercator_quad().extended_to(zoom)?;
    let resolution = tile_matrix_set.resolution(zoom)?;
    let forward = Transform::new("EPSG:4326", "EPSG:3857")?;
    let inverse = forward.inverse()?;
    let latitude = map_xyz
        .latitude
        .clamp(-ogc_tms::MERCATOR_LAT_LIMIT, ogc_tms::MERCATOR_LAT_LIMIT);
    let center_xy = coordinate_to_xy(&forward, (map_xyz.longitude, latitude))?;

    Ok((center_xy, resolution, forward, inverse))
}

/// MapXYZ が有効かどうかを検証する。
pub fn validate_map_xyz(xyz: &MapXYZ) -> bool {
    if !xyz.latitude.is_finite() || !(-90.0..=90.0).contains(&xyz.latitude) {
        return false;
    }

    if !xyz.longitude.is_finite() || !(-180.0..=180.0).contains(&xyz.longitude) {
        return false;
    }

    xyz.zoom <= 30
}

/// MoveXY が有効かどうかを検証する。
pub fn validate_move(movement: &MoveXY) -> bool {
    movement.move_x.is_finite() && movement.move_y.is_finite()
}

/// Screen が有効かどうかを検証する。
pub fn validate_screen(screen: &Screen) -> bool {
    screen.width.is_finite()
        && screen.height.is_finite()
        && screen.width > 0.0
        && screen.height > 0.0
}

/// マップオブジェクトを作成する。
///
/// movement は画面上のドラッグ移動量 (px)。
/// 右方向・下方向へのドラッグを正の値とする。
pub fn create_map_obj(input: &MapEngineInput) -> Result<MapObject, Box<dyn std::error::Error>> {
    let xyz = input.map_xyz;
    let screen = input.screen;
    let movement = input.movement;

    // 入力値の検証
    if !validate_map_xyz(&xyz) {
        return Err("緯度、経度、ズームレベルが無効です".into());
    }

    if !validate_move(&movement) {
        return Err("不正なピクセル移動量です".into());
    }

    if !validate_screen(&screen) {
        return Err("Screenの大きさが不正です".into());
    }

    let zoom = xyz.zoom as u8;

    // XYZ / WebMercatorQuad
    let base_tms = ogc_tms::web_mercator_quad();
    let tms: TileMatrixSet = base_tms.extended_to(zoom)?;

    // WGS84 <-> Web Mercator
    let forward = Transform::new("EPSG:4326", "EPSG:3857")?;
    let inverse = forward.inverse()?;

    // Web Mercator の有効な緯度範囲に制限
    let lat = xyz
        .latitude
        .clamp(-ogc_tms::MERCATOR_LAT_LIMIT, ogc_tms::MERCATOR_LAT_LIMIT);

    // 移動前の中心座標を投影
    let (cx, cy) = coordinate_to_xy(&forward, (xyz.longitude, lat))?;

    // 解像度 (m/px)
    let resolution = tms.resolution(zoom)?;

    // ドラッグ移動量を投影座標に反映
    // 右ドラッグ: 中心は西へ移動
    // 下ドラッグ: 中心は北へ移動
    let moved_cx = cx - movement.move_x * resolution;
    let moved_cy = cy + movement.move_y * resolution;

    // 移動後の中心座標を WGS84 に変換
    let (center_lon, center_lat) = xy_to_coordinate(&inverse, (moved_cx, moved_cy))?;

    let center = [center_lon, center_lat];

    // 表示範囲を計算
    let half_w = screen.width * resolution / 2.0;
    let half_h = screen.height * resolution / 2.0;

    let left = moved_cx - half_w;
    let right = moved_cx + half_w;
    let bottom = moved_cy - half_h;
    let top = moved_cy + half_h;

    let projected_bbox = BoundingBox {
        left,
        bottom,
        right,
        top,
    };

    // 表示範囲に交差するタイルを列挙
    let mut tiles: Vec<TileZXY> = tms
        .tiles(projected_bbox, zoom..=zoom)?
        .map(|tile| -> Result<TileZXY, std::num::TryFromIntError> {
            Ok(TileZXY {
                x: u32::try_from(tile.x)?,
                y: u32::try_from(tile.y)?,
                z: u32::try_from(tile.z)?,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    // 上から下、左から右の順番にする
    tiles.sort_by_key(|tile| (tile.y, tile.x));

    // 左上の座標を含むタイル
    let first_tile = tms.xy_tile(left, top, zoom)?;
    let first_bounds = tms.xy_bounds(first_tile)?;

    // 最初のタイルの画面上の配置位置 (px)
    let offset = [
        (first_bounds.left - left) / resolution,
        (top - first_bounds.top) / resolution,
    ];

    // 表示範囲の対角の座標を WGS84 に変換
    let (min_lon, min_lat) = xy_to_coordinate(&inverse, (left, bottom))?;
    let (max_lon, max_lat) = xy_to_coordinate(&inverse, (right, top))?;

    let bbox = [min_lon, min_lat, max_lon, max_lat];

    Ok(MapObject {
        center,
        bbox,
        tiles,
        offset,
    })
}

#[cfg(test)]
mod tests {
    use super::{MapXYZ, Screen, lon_lat_to_pixel_xy, pixel_xy_to_lon_lat};

    #[test]
    fn pixel_coordinate_conversion_round_trips() {
        let map_xyz = MapXYZ {
            latitude: 35.6812,
            longitude: 139.7671,
            zoom: 12,
        };
        let screen = Screen {
            width: 1280.0,
            height: 800.0,
        };
        let pixel_xy = (878.5, 282.75);

        let (longitude, latitude) = pixel_xy_to_lon_lat(pixel_xy, &map_xyz, screen).unwrap();
        let converted_pixel =
            lon_lat_to_pixel_xy(longitude, latitude, &map_xyz, screen).unwrap();

        assert!((converted_pixel.0 - pixel_xy.0).abs() < 1e-6);
        assert!((converted_pixel.1 - pixel_xy.1).abs() < 1e-6);
    }
}
