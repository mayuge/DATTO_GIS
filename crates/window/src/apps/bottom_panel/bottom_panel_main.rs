use crate::apps::store::map_engine_store::{MapEngineStore, MapEngineStoreTrait};
use crate::apps::store::map_layer_store::{MapLayerStore, MapLayerStoreTrait};
use domain::types::map_layer_type::MapLayer;
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
    /// * `map_layer_store` - マップレイヤーの状態を管理するストア
    pub fn use_bottom_panel_main(
        &mut self,
        ui: &mut egui::Ui,
        engine_store: &MapEngineStore,
        map_layer_store: &MapLayerStore,
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
                let mut attributions: Vec<&(String, String)> = Vec::new();
                for map_layer in map_layer_store.get_all_layers() {
                    let MapLayer::RasterTile(layer) = map_layer else {
                        continue;
                    };
                    if !layer.visible || layer.opacity <= 0.0 {
                        continue;
                    }
                    if let Some(attribution) = layer.attribution.as_ref() {
                        if !attributions.contains(&attribution) {
                            attributions.push(attribution);
                        }
                    }
                }

                if !attributions.is_empty() {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        for (index, (label, url)) in attributions.iter().rev().enumerate() {
                            if index > 0 {
                                ui.label("|");
                            }
                            if url.is_empty() {
                                ui.label(label);
                            } else {
                                ui.hyperlink_to(if label.is_empty() { url } else { label }, url);
                            }
                        }
                    });
                }
            });
        });
    }
}
