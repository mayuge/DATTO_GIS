pub use domain::types::gis_engine_type::SetPictureLayerInput;
use domain::types::map_layer_type::Rect;

/// 地図中心に画像を配置したときの四隅を返す。
///
/// 各座標は (経度, 緯度) で返す。
pub fn set_picture_layer(
    input: SetPictureLayerInput,
) -> Result<Rect, Box<dyn std::error::Error>> {
    if !crate::validate_map_xyz(&input.map_xyz) {
        return Err("画像の中心座標またはズームレベルが無効です".into());
    }
    if !crate::validate_screen(&input.picture_size) {
        return Err("画像サイズが無効です".into());
    }
    if !crate::validate_screen(&input.map_screen_size) {
        return Err("地図画面のサイズが無効です".into());
    }

    let left = (input.map_screen_size.width - input.picture_size.width) / 2.0;
    let right = (input.map_screen_size.width + input.picture_size.width) / 2.0;
    let top = (input.map_screen_size.height - input.picture_size.height) / 2.0;
    let bottom = (input.map_screen_size.height + input.picture_size.height) / 2.0;

    let (top_left_lon, top_left_lat) = crate::pixel_xy_to_lon_lat(
        (left, top),
        &input.map_xyz,
        input.map_screen_size,
    )?;
    let (top_right_lon, top_right_lat) = crate::pixel_xy_to_lon_lat(
        (right, top),
        &input.map_xyz,
        input.map_screen_size,
    )?;
    let (bottom_right_lon, bottom_right_lat) = crate::pixel_xy_to_lon_lat(
        (right, bottom),
        &input.map_xyz,
        input.map_screen_size,
    )?;
    let (bottom_left_lon, bottom_left_lat) = crate::pixel_xy_to_lon_lat(
        (left, bottom),
        &input.map_xyz,
        input.map_screen_size,
    )?;

    Ok(Rect {
        top_left: (top_left_lon, top_left_lat),
        top_right: (top_right_lon, top_right_lat),
        bottom_left: (bottom_left_lon, bottom_left_lat),
        bottom_right: (bottom_right_lon, bottom_right_lat),
    })
}