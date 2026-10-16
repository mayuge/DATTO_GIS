use crate::apps::dialogs::add_image_file_dialog::AddImageFileDialog;
use crate::apps::dialogs::add_raster_tile_dialog::AddRasterTileDialog;
use crate::apps::store::map_engine_store::{MapEngineStore, MapEngineStoreTrait};
use crate::apps::store::panel_visible_store::{PanelVisibleStore, PanelVisibleStoreTrait};
use crate::apps::store::map_layer_store::{MapLayerStore, MapLayerStoreTrait};
use domain::types::map_layer_type::MapLayer;
use domain::types::gis_engine_type::MapXYZ;
use eframe::egui;
use std::sync::{Arc, Mutex};

pub struct TopPanelMain {
    add_image_file_dialog: AddImageFileDialog,
    add_raster_tile_dialog: AddRasterTileDialog,
    search_text: String,
    search_result: Arc<Mutex<Option<Result<MapXYZ, String>>>>,
    search_error: Option<String>,
}

impl Default for TopPanelMain {
    fn default() -> Self {
        Self {
            add_image_file_dialog: AddImageFileDialog::default(),
            add_raster_tile_dialog: AddRasterTileDialog::default(),
            search_text: String::new(),
            search_result: Arc::new(Mutex::new(None)),
            search_error: None,
        }
    }
}

impl TopPanelMain {
    /// トップパネルのUIを描画し、ユーザーの操作を処理する。
    /// # 引数
    /// - `ui`: eguiのUIコンテキスト。
    /// - `panel_visible_store`: パネルの表示状態を管理するストア。
    /// - `map_layer_store`: マップレイヤーを管理するストア。
    pub fn use_top_panel_main(
        &mut self,
        ui: &mut egui::Ui,
        panel_visible_store: &mut PanelVisibleStore,
        map_layer_store: &mut MapLayerStore,
        map_engine_store: &mut MapEngineStore,
    ) {
        let completed_search = self.search_result.lock().unwrap().take();
        if let Some(result) = completed_search {
            match result {
                Ok(map_xyz) => {
                    map_engine_store.set_xyz(map_xyz);
                    self.search_error = None;
                }
                Err(error) => self.search_error = Some(error),
            }
        }

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
                            self.add_raster_tile_dialog.open(panel_visible_store);
                        }
                        if ui.button("画像を追加").clicked() {
                            self.add_image_file_dialog.open();
                        }

                    });

                    ui.menu_button("レイアウト", |ui| {
                        if ui.button("サイドパネル表示切替").clicked() {
                            panel_visible_store.toggle_side_panel();
                        }
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let input_response = ui.add(
                            egui::TextEdit::singleline(&mut self.search_text)
                                .hint_text("住所を検索")
                                .desired_width(260.0),
                        );
                        let enter_pressed = input_response.lost_focus()
                            && ui.input(|input| input.key_pressed(egui::Key::Enter));

                        let address = self.search_text.trim().to_owned();
                        if enter_pressed && !address.is_empty() {
                            self.search_text.clear();
                            self.search_error = None;
                            let search_result = Arc::clone(&self.search_result);
                            let ctx = ui.ctx().clone();
                            geocoding::api::index::geocode_address(address, move |result| {
                                *search_result.lock().unwrap() = Some(result);
                                ctx.request_repaint();
                            });
                        }
                    });
                });
            });
        });

        let map_engine_input = map_engine_store.get_map_engine_input();
        if let Some(mut layer) = self.add_image_file_dialog.use_add_image_file_dialog(
            ui.ctx(),
            *map_engine_store.get_xyz(),
            map_engine_input.screen,
        ) {
            layer.id = Self::next_layer_id(map_layer_store, "picture_layer");
            map_layer_store.add_layer(MapLayer::Picture(layer));
        }

        if let Some(mut layer) = self
            .add_raster_tile_dialog
            .use_add_raster_tile_dialog(ui.ctx(), panel_visible_store)
        {
            layer.id = Self::next_layer_id(map_layer_store, "custom_raster_tile");
            map_layer_store.add_layer(MapLayer::RasterTile(layer));
        }
    }

    /// 次のラスタタイルレイヤーIDを生成する。
    /// # 引数
    /// - `map_layer_store`: 現在のマップレイヤーストア。
    ///
    /// # 戻り値
    /// 新しいラスタタイルレイヤーID。
    fn next_layer_id(map_layer_store: &MapLayerStore, prefix: &str) -> String {
        let mut index = 1;
        loop {
            let id = format!("{prefix}_{index}");
            if !map_layer_store
                .is_layer_id(&id)
            {
                return id;
            }
            index += 1;
        }
    }
}
