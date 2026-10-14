use crate::apps::store::map_engine_store::{MapEngineStore, MapEngineStoreTrait};
use domain::params::map_config::{
    MAP_CENTER_LATITUDE, MAP_CENTER_LONGITUDE, MAP_MAX_ZOOM_LEVEL, MAP_MIN_ZOOM_LEVEL,
    MAP_ZOOM_LEVEL,
};
use domain::types::gis_engine_type::MapXYZ;
use eframe::egui;

#[derive(Default)]
pub struct TopPanelMain;

impl TopPanelMain {
    pub fn use_top_panel_main(
        &mut self,
        ui: &mut egui::Ui,
        engine_store: &mut MapEngineStore,
        side_panel_visible: &mut bool,
    ) {
        egui::Panel::top("top_panel").show(ui, |ui| {
            let menu_visuals = ui.visuals().clone();
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
                    ui.menu_button("メニュー", |ui| {
                        let zoom = engine_store.get_zoom();

                        if ui.button("拡大").clicked() && zoom < MAP_MAX_ZOOM_LEVEL {
                            engine_store.set_zoom(zoom + 1);
                        }
                        if ui.button("縮小").clicked() && zoom > MAP_MIN_ZOOM_LEVEL {
                            engine_store.set_zoom(zoom - 1);
                        }
                    });

                    ui.menu_button("レイアウト", |ui| {
                        *ui.visuals_mut() = menu_visuals.clone();
                        let label = if *side_panel_visible {
                            "レイヤーパネルを隠す"
                        } else {
                            "レイヤーパネルを表示"
                        };

                        if ui.button(label).clicked() {
                            *side_panel_visible = !*side_panel_visible;
                        }
                    });
                });
            });
        });
    }
}
