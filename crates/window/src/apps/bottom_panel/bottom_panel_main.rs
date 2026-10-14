use crate::apps::store::map_engine_store::{MapEngineStore, MapEngineStoreTrait};
use crate::apps::store::raster_tile_layer_store::{
    RasterTileLayerStore, RasterTileLayerStoreTrait,
};
use eframe::egui;

/// 下部のステータスバーを管理する。
#[derive(Default)]
pub struct BottomPanelMain;

impl BottomPanelMain {
    /// 現在の地図の中心座標・ズームレベル・表示範囲を描画する。
    ///
    /// # Arguments
    /// * `ui` - 描画するUIコンテキスト
    /// * `engine_store` - 地図エンジンの状態を管理するストア
    /// * `raster_tile_layer_store` - ラスタータイルレイヤーの状態を管理するストア
    pub fn use_bottom_panel_main(
        &mut self,
        ui: &mut egui::Ui,
        engine_store: &MapEngineStore,
        raster_tile_layer_store: &RasterTileLayerStore,
    ) {
        egui::Panel::bottom("bottom_panel").show(ui, |ui| {
            ui.horizontal(|ui| {
                let xyz = engine_store.get_xyz();

                ui.label(format!(
                    "緯度 {:.6}  経度 {:.6}",
                    xyz.latitude, xyz.longitude
                ));
                ui.separator();
                ui.label(format!("ズーム {}", xyz.zoom));
                ui.separator();

                // bbox: [min_lon, min_lat, max_lon, max_lat]
                match engine_store.get_map_object() {
                    Some(map_object) => {
                        let [min_lon, min_lat, max_lon, max_lat] = map_object.bbox;
                        ui.label(format!(
                            "BBOX {:.6}, {:.6}, {:.6}, {:.6}",
                            min_lon, min_lat, max_lon, max_lat
                        ));
                    }
                    None => {
                        ui.label("BBOX -");
                    }
                }

                // 表示中のレイヤーの出典を重複なしで右寄せ表示する。
                let mut attributions: Vec<&str> = Vec::new();
                for layer in raster_tile_layer_store.get_all_raster_tile_layers() {
                    if !layer.visible || layer.opacity <= 0.0 {
                        continue;
                    }
                    if let Some(attribution) = layer.attribution.as_deref() {
                        if !attributions.contains(&attribution) {
                            attributions.push(attribution);
                        }
                    }
                }

                if !attributions.is_empty() {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(attributions.join(" | "));
                    });
                }
            });
        });
    }
}
