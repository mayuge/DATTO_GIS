use crate::apps::store::raster_tile_layer_store::{
    RasterTileLayerStore, RasterTileLayerStoreTrait,
};
use eframe::egui;

/// 左側のサイドパネルを管理する。
#[derive(Default)]
pub struct SidePanelMain;

impl SidePanelMain {
    /// サイドパネルを描画する。
    ///
    /// # Arguments
    /// * `ui` - 描画するUIコンテキスト
    /// * `raster_tile_layer_store` - ラスタータイルレイヤーの状態を管理するストア
    ///
    pub fn use_side_panel_main(
        &mut self,
        ui: &mut egui::Ui,
        raster_tile_layer_store: &mut RasterTileLayerStore,
    ) {
        egui::Panel::left("side_panel")
            .default_size(240.0)
            .min_size(180.0)
            .resizable(true)
            .show(ui, |ui| {
                ui.heading("レイヤー");

                // 描画中にストアを変更できるよう、必要な値を複製する。
                let layers: Vec<(String, String, bool, f32)> = raster_tile_layer_store
                    .get_all_raster_tile_layers()
                    .iter()
                    .map(|layer| {
                        (
                            layer.id.clone(),
                            layer.name.clone(),
                            layer.visible,
                            layer.opacity,
                        )
                    })
                    .collect();

                if layers.is_empty() {
                    ui.label("レイヤーがありません");
                    return;
                }

                for (id, name, mut visible, mut opacity) in layers {
                    ui.push_id(&id, |ui| {
                        // 表示・非表示を切り替える。
                        if ui.checkbox(&mut visible, &name).changed() {
                            raster_tile_layer_store.toggle_visibility(&id);
                        }

                        ui.label("不透明度");

                        // 不透明度を設定する。
                        if ui
                            .add(egui::Slider::new(&mut opacity, 0.0..=1.0).show_value(true))
                            .changed()
                        {
                            raster_tile_layer_store.set_opacity(&id, opacity);
                        }

                        ui.separator();
                    });
                }
            });
    }
}
