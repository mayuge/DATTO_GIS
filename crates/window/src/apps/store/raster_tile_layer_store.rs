use domain::types::map_layer_type::RasterTileLayer;

pub struct RasterTileLayerStore {
    layers: Vec<RasterTileLayer>,
}

pub trait RasterTileLayerStoreTrait {
    fn toggle_visibility(&mut self, layer_id: &str);
    fn set_opacity(&mut self, layer_id: &str, opacity: f32);
    fn add_layer(&mut self, layer: RasterTileLayer);
    fn remove_layer(&mut self, layer_id: &str);
    fn get_all_raster_tile_layers(&self) -> &[RasterTileLayer];
}

impl RasterTileLayerStoreTrait for RasterTileLayerStore {
    /// レイヤーの表示/非表示を切り替える。
    ///
    /// # Arguments
    /// * `layer_id` - 切り替えるレイヤーのID

    fn toggle_visibility(&mut self, layer_id: &str) {
        if let Some(layer) = self.layers.iter_mut().find(|layer| layer.id == layer_id) {
            layer.visible = !layer.visible;
        }
    }

    /// レイヤーの不透明度を設定する。
    ///
    /// # Arguments
    /// * `layer_id` - 不透明度を設定するレイヤーのID
    /// * `opacity` - 設定する不透明度（0.0〜1.0）
    ///
    fn set_opacity(&mut self, layer_id: &str, opacity: f32) {
        if let Some(layer) = self.layers.iter_mut().find(|layer| layer.id == layer_id) {
            layer.opacity = opacity.clamp(0.0, 1.0);
        }
    }

    /// ラスタータイルレイヤーを追加する。
    ///
    /// # Arguments
    /// * `layer` - 追加するラスタータイルレイヤー
    fn add_layer(&mut self, layer: RasterTileLayer) {
        self.layers.push(layer);
    }

    /// ラスタータイルレイヤーを削除する。
    ///
    /// # Arguments
    /// * `layer_id` - 削除するラスタータイルレイヤーのID
    fn remove_layer(&mut self, layer_id: &str) {
        self.layers.retain(|layer| layer.id != layer_id);
    }

    /// すべてのラスタータイルレイヤーを取得する。
    ///
    /// # Returns
    /// * `&[RasterTileLayer]` - すべてのラスタータイルレイヤーのスライス
    fn get_all_raster_tile_layers(&self) -> &[RasterTileLayer] {
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
