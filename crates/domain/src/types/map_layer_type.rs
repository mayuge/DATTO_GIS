#[derive(Clone, Debug, PartialEq)]
pub struct RasterTileLayer {
    pub id: String,
    pub name: String,
    pub url: String,
    pub opacity: f32,
    pub visible: bool,
    pub attribution: Option<String>,
}
