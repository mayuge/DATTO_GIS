#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapXYZ {
    pub latitude: f64,
    pub longitude: f64,
    pub zoom: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoveXY {
    pub move_x: f64,
    pub move_y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Screen {
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileZXY {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MapObject {
    // WGS84: [longitude, latitude]
    pub center: [f64; 2],

    // WGS84: [min_lon, min_lat, max_lon, max_lat]
    pub bbox: [f64; 4],

    pub tiles: Vec<TileZXY>,

    // 最初のタイルの画面上の配置位置 (px)
    pub delta: [f64; 2],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MapEngineInput {
    pub map_xyz: MapXYZ,
    pub screen: Screen,
    pub movement: MoveXY,
}
