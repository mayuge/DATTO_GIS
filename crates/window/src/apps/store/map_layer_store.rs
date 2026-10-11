use domain::RasterTileLayer;

pub struct MapLayerStore {
    layers: Vec<RasterTileLayer>,
}

pub trait MapLayerStoreTrait {
    fn add_layer(&mut self, layer: RasterTileLayer);
    fn remove_layer(&mut self, layer_id: &str);
    fn get_layers(&self) -> &[RasterTileLayer];
}

impl MapLayerStoreTrait for MapLayerStore {
    fn add_layer(&mut self, layer: RasterTileLayer) {
        self.layers.push(layer);
    }

    fn remove_layer(&mut self, layer_id: &str) {
        self.layers.retain(|layer| layer.id != layer_id);
    }

    fn get_layers(&self) -> &[RasterTileLayer] {
        &self.layers
    }
}

impl Default for MapLayerStore {
    fn default() -> Self {
        Self {
            layers: vec![RasterTileLayer {
                id: "openstreetmap".to_owned(),
                name: "OpenStreetMap".to_owned(),
                url: "https://tile.openstreetmap.org/{z}/{x}/{y}.png".to_owned(),
                opacity: 1.0,
                visible: true,
                attribution: Some("© OpenStreetMap contributors".to_owned()),
            }],
        }
    }
}
