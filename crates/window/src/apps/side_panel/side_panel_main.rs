use crate::apps::store::map_layer_store::{MapLayerStore, MapLayerStoreTrait};
use domain::params::design_token_config::{LAYER_LIST_HEIGHT, SIDE_PANEL_WIDTH};
use eframe::egui;

/// 左側のサイドパネルを管理する。
#[derive(Default)]
pub struct SidePanelMain;

impl SidePanelMain {
    /// サイドパネルを描画する。
    ///
    /// # Arguments
    /// * `ui` - 描画するUIコンテキスト
    /// * `map_layer_store` - マップレイヤーの状態を管理するストア
    ///
    pub fn use_side_panel_main(
        &mut self,
        ui: &mut egui::Ui,
        map_layer_store: &mut MapLayerStore,
    ) {
        egui::Panel::left("side_panel")
            .default_size(SIDE_PANEL_WIDTH)
            .min_size(SIDE_PANEL_WIDTH)
            .resizable(true)
            .show(ui, |ui| {
                // 表示中にストアを変更しても安全なように、
                // レイヤー情報をコピーしておく。
                let layers = map_layer_store.get_layer_list();

                if layers.is_empty() {
                    ui.label("レイヤーがありません");
                    return;
                }

                let mut move_positions: Option<(usize, usize)> = None;
                let layer_visuals = ui.visuals().clone();

                ui.scope(|ui| {
                    let visuals = &mut ui.visuals_mut().widgets;
                    visuals.active.bg_fill = egui::Color32::TRANSPARENT;
                    visuals.active.bg_stroke = egui::Stroke::NONE;
                    visuals.inactive.bg_fill = egui::Color32::TRANSPARENT;
                    visuals.inactive.bg_stroke = egui::Stroke::NONE;

                    for (position, (id, name, mut visible, mut opacity)) in
                        layers.into_iter().enumerate()
                    {
                        let (_, dropped_payload) =
                            ui.dnd_drop_zone::<usize, ()>(egui::Frame::NONE, |ui| {
                                *ui.visuals_mut() = layer_visuals.clone();
                                ui.set_min_height(LAYER_LIST_HEIGHT);

                                ui.push_id(&id, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.scope(|ui| {
                                            let widgets = &mut ui.visuals_mut().widgets;
                                            widgets.inactive.bg_fill = egui::Color32::from_gray(85);
                                            widgets.inactive.bg_stroke = egui::Stroke::new(
                                                1.5,
                                                egui::Color32::from_gray(175),
                                            );
                                            widgets.inactive.fg_stroke =
                                                egui::Stroke::new(1.5, egui::Color32::WHITE);
                                            widgets.hovered.bg_fill = egui::Color32::from_gray(105);
                                            widgets.hovered.bg_stroke =
                                                egui::Stroke::new(1.5, egui::Color32::WHITE);
                                            widgets.hovered.fg_stroke =
                                                egui::Stroke::new(1.75, egui::Color32::WHITE);
                                            widgets.active.bg_fill = egui::Color32::from_gray(120);
                                            widgets.active.bg_stroke =
                                                egui::Stroke::new(1.5, egui::Color32::WHITE);
                                            widgets.active.fg_stroke =
                                                egui::Stroke::new(2.0, egui::Color32::WHITE);

                                            ui.vertical(|ui| {
                                                let visibility_response =
                                                    ui.checkbox(&mut visible, &name);
                                                if visibility_response.changed() {
                                                    map_layer_store.toggle_visibility(&id);
                                                }
                                                Self::show_layer_context_menu(
                                                    &visibility_response,
                                                    map_layer_store,
                                                    &id,
                                                );

                                                let opacity_response = ui.add(
                                                    egui::Slider::new(&mut opacity, 0.0..=1.0)
                                                        .show_value(false),
                                                );
                                                if opacity_response.changed() {
                                                    map_layer_store.set_opacity(&id, opacity);
                                                }
                                                Self::show_layer_context_menu(
                                                    &opacity_response,
                                                    map_layer_store,
                                                    &id,
                                                );
                                            });
                                        });

                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                let drag_response = ui.dnd_drag_source(
                                                    egui::Id::new(("raster_layer_drag", &id)),
                                                    position,
                                                    |ui| {
                                                        ui.label(
                                                            egui::RichText::new("\u{E945}").font(
                                                                egui::FontId::new(
                                                                    16.0,
                                                                    egui::FontFamily::Name(
                                                                        "material_icons".into(),
                                                                    ),
                                                                ),
                                                            ),
                                                        );
                                                    },
                                                );
                                                Self::show_layer_context_menu(
                                                    &drag_response.response,
                                                    map_layer_store,
                                                    &id,
                                                );
                                            },
                                        );
                                    });
                                });
                            });

                        if let Some(payload) = dropped_payload {
                            let from_position = *payload;
                            if from_position != position {
                                move_positions = Some((from_position, position));
                            }
                        }

                        ui.separator();
                    }
                });

                // 一覧の描画が終わってから並び替える。
                if let Some((from_position, to_position)) = move_positions {
                    map_layer_store.move_layer(from_position, to_position);
                }
            });
    }

    /// レイヤーのコンテキストメニューを表示する
    /// # 引数
    /// - `response`: コンテキストメニューを表示する対象の `egui::Response`
    /// - `map_layer_store`: レイヤーの管理を行う `MapLayerStore`
    /// - `layer_id`: コンテキストメニューを表示するレイヤーのID
    fn show_layer_context_menu(
        response: &egui::Response,
        map_layer_store: &mut MapLayerStore,
        layer_id: &str,
    ) {
        response.context_menu(|ui| {
            if ui.button("レイヤーを削除").clicked() {
                map_layer_store.remove_layer(layer_id);
                ui.close();
            }
        });
    }
}
