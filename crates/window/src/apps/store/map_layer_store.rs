use domain::types::map_layer_type::{MapLayer, MapLayers, RasterTileLayer, Rect};

pub struct MapLayerStore {
    map_layers: MapLayers,
}

pub trait MapLayerStoreTrait {
    fn toggle_visibility(&mut self, layer_id: &str);
    fn set_opacity(&mut self, layer_id: &str, opacity: f32);
    fn add_layer(&mut self, layer: MapLayer);
    fn remove_layer(&mut self, layer_id: &str);
    fn get_all_layers(&self) -> &[MapLayer];
    fn update_picture_rect(&mut self, layer_id: &str, picture_rect: Rect) -> bool;
    fn get_layer_list(&self) -> Vec<(String, String, bool, f32)>;
    fn is_layer_id(&self, layer_id: &str) -> bool;
    fn move_layer(&mut self, from_index: usize, to_index: usize);
}

impl MapLayerStoreTrait for MapLayerStore {
    ///表示切替
    /// 
    /// layer_id: 表示を切り替えるレイヤーのID。
    fn toggle_visibility(&mut self, layer_id: &str) {
        if let Some(layer) = self
            .map_layers
            .layers
            .iter_mut()
            .find(|layer| get_layer_id(layer) == layer_id)
        {
            match layer {
                MapLayer::RasterTile(layer) => layer.visible = !layer.visible,
                MapLayer::Picture(layer) => layer.visible = !layer.visible,
            }
        }
    }

    /// 不透明度設定
    ///
    /// layer_id: 不透明度を設定するレイヤーのID。
    /// opacity: 設定する不透明度の値（0.0〜1.0）。
    fn set_opacity(&mut self, layer_id: &str, opacity: f32) {
        if let Some(layer) = self
            .map_layers
            .layers
            .iter_mut()
            .find(|layer| get_layer_id(layer) == layer_id)
        {
            let opacity = opacity.clamp(0.0, 1.0);
            match layer {
                MapLayer::RasterTile(layer) => layer.opacity = opacity,
                MapLayer::Picture(layer) => layer.opacity = opacity,
            }
        }
    }

    /// レイヤー追加
    ///
    /// layer: 追加するレイヤー。
    fn add_layer(&mut self, layer: MapLayer) {
        //先頭に追加する
        self.map_layers.layers.insert(0, layer);
    }

    /// レイヤー削除
    ///
    /// layer_id: 削除するレイヤーのID。
    fn remove_layer(&mut self, layer_id: &str) {
        self.map_layers
            .layers
            .retain(|layer| get_layer_id(layer) != layer_id);
    }

    /// 全レイヤー取得
    ///
    /// 戻り値: 全てのレイヤーのスライス。
    fn get_all_layers(&self) -> &[MapLayer] {
        &self.map_layers.layers
    }

    fn update_picture_rect(&mut self, layer_id: &str, picture_rect: Rect) -> bool {
        if let Some(MapLayer::Picture(layer)) = self
            .map_layers
            .layers
            .iter_mut()
            .find(|layer| matches!(layer, MapLayer::Picture(picture) if picture.id == layer_id))
        {
            layer.picture_rect = picture_rect;
            true
        } else {
            false
        }
    }


    /// UI用のレイヤー一覧取得
    /// 戻り値: レイヤーのID、名前、表示状態、不透明度のタプルのベクター。
    /// # 戻り値
    /// - レイヤーのID、名前、表示状態、不透明度のタプルのベクター。
    fn get_layer_list(&self) -> Vec<(String, String, bool, f32)> {
        self.map_layers
            .layers
            .iter()
            .map(|layer| match layer {
                MapLayer::RasterTile(layer) => (
                    layer.id.clone(),
                    layer.name.clone(),
                    layer.visible,
                    layer.opacity,
                ),
                MapLayer::Picture(layer) => (
                    layer.id.clone(),
                    layer.name.clone(),
                    layer.visible,
                    layer.opacity,
                ),
            })
            .collect()
    }

    /// レイヤーID存在確認
    /// # 引数
    /// - `layer_id`: 存在確認するレイヤーのID。
    ///
    /// # 戻り値
    /// - `true`: レイヤーIDが存在する場合。
    /// - `false`: レイヤーIDが存在しない場合。
    fn is_layer_id(&self, layer_id: &str) -> bool {
        self.map_layers
            .layers
            .iter()
            .any(|layer| get_layer_id(layer) == layer_id)
    }

    /// レイヤー移動
    ///
    /// from_index: 移動元のインデックス。
    /// to_index: 移動先のインデックス。
    fn move_layer(&mut self, from_index: usize, to_index: usize) {
        let len = self.map_layers.layers.len();
        if from_index >= len || to_index >= len || from_index == to_index {
            return;
        }

        let layer = self.map_layers.layers.remove(from_index);
        self.map_layers.layers.insert(to_index, layer);
    }
}

/// レイヤーID取得
/// # 引数
/// - `layer`: レイヤー。
///
/// # 戻り値
/// - レイヤーのID。
fn get_layer_id(layer: &MapLayer) -> &str {
    match layer {
        MapLayer::RasterTile(layer) => &layer.id,
        MapLayer::Picture(layer) => &layer.id,
    }
}

impl Default for MapLayerStore {
    fn default() -> Self {
        Self {
            map_layers: MapLayers {
                layers: vec![
                    MapLayer::RasterTile(RasterTileLayer {
                        id: String::from("gsi_std"),
                        name: String::from("GSI Standard"),
                        url: String::from(
                            "https://cyberjapandata.gsi.go.jp/xyz/std/{z}/{x}/{y}.png",
                        ),
                        opacity: 1.0,
                        visible: true,
                        min_zoom: Some(0),
                        max_zoom: Some(18),
                        attribution: Some((
                            String::from("地理院タイル"),
                            String::from("https://maps.gsi.go.jp/development/ichiran.html"),
                        )),
                    }),
                    MapLayer::RasterTile(RasterTileLayer {
                        id: String::from("openstreetmap"),
                        name: String::from("OpenStreetMap"),
                        url: String::from("https://tile.openstreetmap.org/{z}/{x}/{y}.png"),
                        opacity: 1.0,
                        visible: true,
                        min_zoom: Some(0),
                        max_zoom: Some(19),
                        attribution: Some((
                            String::from("© OpenStreetMap contributors"),
                            String::from("https://www.openstreetmap.org/copyright"),
                        )),
                    }),
                    MapLayer::RasterTile(RasterTileLayer {
                        id: String::from("gsi_pale"),
                        name: String::from("GSI Pale"),
                        url: String::from(
                            "https://cyberjapandata.gsi.go.jp/xyz/pale/{z}/{x}/{y}.png",
                        ),
                        opacity: 1.0,
                        visible: true,
                        min_zoom: Some(0),
                        max_zoom: Some(18),
                        attribution: Some((
                            String::from("地理院タイル"),
                            String::from("https://maps.gsi.go.jp/development/ichiran.html"),
                        )),
                    }),
                ],
            },
        }
    }
}