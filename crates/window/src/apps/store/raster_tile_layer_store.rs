use domain::types::map_layer_type::RasterTileLayer;

pub struct RasterTileLayerStore {
    layers: Vec<RasterTileLayer>,
}

pub trait RasterTileLayerStoreTrait {
    fn add_layer(&mut self, layer: RasterTileLayer);
    fn remove_layer(&mut self, layer_id: &str);
    fn get_all(&self) -> &[RasterTileLayer];
}

impl RasterTileLayerStoreTrait for RasterTileLayerStore {
    fn add_layer(&mut self, layer: RasterTileLayer) {
        self.layers.push(layer);
    }

    fn remove_layer(&mut self, layer_id: &str) {
        self.layers.retain(|layer| layer.id != layer_id);
    }

    fn get_all(&self) -> &[RasterTileLayer] {
        &self.layers
    }
}

impl Default for RasterTileLayerStore {
    fn default() -> Self {
        Self {
            layers: vec![
                RasterTileLayer {
                    id: String::from("openstreetmap"),
                    name: String::from("OpenStreetMap"),
                    url: String::from("https://tile.openstreetmap.org/{z}/{x}/{y}.png"),
                    opacity: 0.5,
                    visible: true,
                    attribution: Some(String::from("© OpenStreetMap contributors")),
                },
                RasterTileLayer {
                    id: String::from("gsi_std"),
                    name: String::from("GSI Standard"),
                    url: String::from("https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png"),
                    opacity: 0.5,
                    visible: true,
                    attribution: Some(String::from("地理院タイル")),
                },
            ],
        }
    }
}
