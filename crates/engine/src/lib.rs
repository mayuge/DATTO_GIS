use ogc_tms::{BoundingBox, TileMatrixSet};
use proj_core::Transform;

pub struct MapXYZ {
    pub latitude: f64,
    pub longitude: f64,
    pub zoom: u32,
}

pub struct MoveXY {
    pub move_x: f64,
    pub move_y: f64,
}

pub struct Screen {
    pub width: f64,
    pub height: f64,
}

pub struct TileZXY {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

pub struct MapObject {
    // WGS84: [longitude, latitude]
    pub center: [f64; 2],

    // WGS84: [min_lon, min_lat, max_lon, max_lat]
    pub bbox: [f64; 4],

    pub tiles: Vec<TileZXY>,

    // 最初のタイルの画面上の配置位置 (px)
    pub delta: [f64; 2],
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
pub fn create_map_obj(
    xyz: MapXYZ,
    screen: Screen,
    movement: MoveXY,
) -> Result<MapObject, Box<dyn std::error::Error>> {
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
    let (cx, cy) = forward.convert((xyz.longitude, lat))?;

    // 解像度 (m/px)
    let resolution = tms.resolution(zoom)?;

    // ドラッグ移動量を投影座標に反映
    // 右ドラッグ: 中心は西へ移動
    // 下ドラッグ: 中心は北へ移動
    let moved_cx = cx - movement.move_x * resolution;
    let moved_cy = cy + movement.move_y * resolution;

    // 移動後の中心座標を WGS84 に変換
    let (center_lon, center_lat) = inverse.convert((moved_cx, moved_cy))?;

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
    let tiles: Vec<TileZXY> = tms
        .tiles(projected_bbox, zoom..=zoom)?
        .map(|tile| -> Result<TileZXY, std::num::TryFromIntError> {
            Ok(TileZXY {
                x: u32::try_from(tile.x)?,
                y: u32::try_from(tile.y)?,
                z: u32::try_from(tile.z)?,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    // 左上の座標を含むタイル
    let first_tile = tms.xy_tile(left, top, zoom)?;
    let first_bounds = tms.xy_bounds(first_tile)?;

    // 最初のタイルの画面上の配置位置 (px)
    let delta = [
        (first_bounds.left - left) / resolution,
        (top - first_bounds.top) / resolution,
    ];

    // 表示範囲の対角の座標を WGS84 に変換
    let (min_lon, min_lat) = inverse.convert((left, bottom))?;
    let (max_lon, max_lat) = inverse.convert((right, top))?;

    let bbox = [min_lon, min_lat, max_lon, max_lat];

    Ok(MapObject {
        center,
        bbox,
        tiles,
        delta,
    })
}
