use crate::apps::dialogs::add_raster_tile_dialog::AddRasterTileDialog;
use crate::apps::store::panel_visible_store::{PanelVisibleStore, PanelVisibleStoreTrait};
use crate::apps::store::raster_tile_layer_store::{
    RasterTileLayerStore, RasterTileLayerStoreTrait,
};
use eframe::egui;

#[derive(Default)]
pub struct TopPanelMain {
    add_raster_tile_dialog: AddRasterTileDialog,
}

impl TopPanelMain {
    pub fn use_top_panel_main(
        &mut self,
        ui: &mut egui::Ui,
        panel_visible_store: &mut PanelVisibleStore,
        raster_tile_layer_store: &mut RasterTileLayerStore,
    ) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            ui.scope(|ui| {
                let widgets = &mut ui.visuals_mut().widgets;
                // ボタンの背景は weak_bg_fill で描画されるため、こちらも透明にする。
                widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
                widgets.inactive.bg_stroke = egui::Stroke::NONE;
                for state in [
                    &mut widgets.inactive,
                    &mut widgets.hovered,
                    &mut widgets.active,
                    &mut widgets.open,
                ] {
                    state.bg_stroke = egui::Stroke::NONE;
                }

                ui.horizontal(|ui| {
                    ui.menu_button("レイヤー", |ui| {
                        if ui.button("ベースマップを追加").clicked() {
                            self.add_raster_tile_dialog.open();
                        }
                    });

                    ui.menu_button("レイアウト", |ui| {
                        if ui.button("サイドパネル表示切替").clicked() {
                            panel_visible_store.toggle_side_panel();
                        }
                    });
                });
            });
        });

        if let Some(mut layer) = self.add_raster_tile_dialog.show(ui.ctx()) {
            layer.id = Self::next_layer_id(raster_tile_layer_store);
            raster_tile_layer_store.add_layer(layer);
        }
    }

    fn next_layer_id(raster_tile_layer_store: &RasterTileLayerStore) -> String {
        let mut index = 1;
        loop {
            let id = format!("custom_raster_tile_{index}");
            if !raster_tile_layer_store
                .get_all_raster_tile_layers()
                .iter()
                .any(|layer| layer.id == id)
            {
                return id;
            }
            index += 1;
        }
    }
}
