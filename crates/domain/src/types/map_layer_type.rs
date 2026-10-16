use crate::types::gis_engine_type::Screen;

#[derive(Clone, Debug, PartialEq)]
pub struct RasterTileLayer {
    pub id: String,
    pub name: String,
    pub url: String,
    pub opacity: f32,
    pub visible: bool,
    pub min_zoom: Option<u8>,
    pub max_zoom: Option<u8>,
    pub attribution: Option<(String, String)>,
}


#[derive(Clone, Debug, PartialEq)]
/// WGS84の四隅。各タプルは (経度, 緯度)。
pub struct Rect {
    pub top_left: (f64, f64),
    pub top_right: (f64, f64),
    pub bottom_left: (f64, f64),
    pub bottom_right: (f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PictureLayer {
    pub id: String,
    pub name: String,
    pub opacity: f32,
    pub visible: bool,
    pub picture_size: Screen,
    pub picture_data: Vec<u8>,
    pub picture_path: String,
    pub picture_rect: Rect,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MapLayer {
    RasterTile(RasterTileLayer),
    Picture(PictureLayer),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MapLayers {
    pub layers: Vec<MapLayer>,
}
