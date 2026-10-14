use crate::apps::store::raster_tile_layer_store::{
    RasterTileLayerStore, RasterTileLayerStoreTrait,
};
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
    /// * `raster_tile_layer_store` - ラスタータイルレイヤーの状態を管理するストア
    ///
    pub fn use_side_panel_main(
        &mut self,
        ui: &mut egui::Ui,
        raster_tile_layer_store: &mut RasterTileLayerStore,
    ) {
        egui::Panel::left("side_panel")
            .default_size(SIDE_PANEL_WIDTH)
            .min_size(SIDE_PANEL_WIDTH)
            .resizable(true)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("レイヤー").size(12.0));
                ui.separator();
                ui.separator();
                ui.label(egui::RichText::new("ベースマップ").size(12.0));
                ui.separator();

                // 表示中にストアを変更しても安全なように、
                // レイヤー情報をコピーしておく。
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
                                                if ui.checkbox(&mut visible, &name).changed() {
                                                    raster_tile_layer_store.toggle_visibility(&id);
                                                }

                                                if ui
                                                    .add(
                                                        egui::Slider::new(&mut opacity, 0.0..=1.0)
                                                            .show_value(false),
                                                    )
                                                    .changed()
                                                {
                                                    raster_tile_layer_store
                                                        .set_opacity(&id, opacity);
                                                }
                                            });
                                        });

                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.dnd_drag_source(
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
                    raster_tile_layer_store.move_layer(from_position, to_position);
                }
            });
    }
}
