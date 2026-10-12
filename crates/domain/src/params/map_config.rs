/// 初期表示中心経度
pub const MAP_CENTER_LONGITUDE: f64 = 139.62286099196345;

/// 初期表示中心緯度
pub const MAP_CENTER_LATITUDE: f64 = 35.70538438612006;

/// 初期ズームレベル
pub const MAP_ZOOM_LEVEL: u32 = 18;

/// Web Mercator（表示座標系）
pub const DISPLAY_PROJ_EPSG: u32 = 3857;

/// WGS84（データ座標系）
pub const DATA_PROJ_EPSG: u32 = 4326;

/// タイル1枚のサイズ（ピクセル）
pub const MAP_TILE_SIZE: f32 = 256.0;

/// 最大ズームレベル
pub const MAP_MAX_ZOOM_LEVEL: u32 = 24;

/// 最小ズームレベル
pub const MAP_MIN_ZOOM_LEVEL: u32 = 0;
