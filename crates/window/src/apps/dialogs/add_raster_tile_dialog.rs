use crate::apps::store::panel_visible_store::{PanelVisibleStore, PanelVisibleStoreTrait};
use domain::types::map_layer_type::RasterTileLayer;
use eframe::egui;

#[derive(Default)]
pub struct AddRasterTileDialog {
    name: String,
    url: String,
    attribution: String,
    attribution_url: String,
    error: Option<String>,
}

enum DialogAction {
    None,
    Close,
    Add(RasterTileLayer),
}

impl AddRasterTileDialog {
    pub fn open(&mut self, panel_visible_store: &mut PanelVisibleStore) {
        panel_visible_store.open_add_raster_tile_dialog();
        self.error = None;
    }

    pub fn use_add_raster_tile_dialog(
        &mut self,
        ctx: &egui::Context,
        panel_visible_store: &mut PanelVisibleStore,
    ) -> Option<RasterTileLayer> {
        if !panel_visible_store.is_add_raster_tile_dialog_open() {
            return None;
        }

        let mut is_open = panel_visible_store.is_add_raster_tile_dialog_open();
        let action = egui::Window::new("ベースマップを追加")
            .open(&mut is_open)
            .resizable(false)
            .show(ctx, |ui| {
                egui::Frame::new().inner_margin(16.0).show(ui, |ui| {
                    ui.label("レイヤー名");
                    ui.text_edit_singleline(&mut self.name);

                    ui.label("タイルURL（{z}/{x}/{y} を含む）");
                    ui.text_edit_singleline(&mut self.url);

                    ui.label("出典（任意）");
                    ui.text_edit_singleline(&mut self.attribution);

                    ui.label("出典リンクURL（任意）");
                    ui.text_edit_singleline(&mut self.attribution_url);

                    if let Some(error) = &self.error {
                        ui.colored_label(ui.visuals().error_fg_color, error);
                    }

                    ui.add_space(16.0);
                    let button_row_width = ui.available_width();
                    let button_row_size = egui::vec2(ui.available_width(), 40.0);
                    ui.allocate_ui_with_layout(
                        button_row_size,
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.add_space(((button_row_width - 212.0) / 2.0).max(0.0));
                            ui.spacing_mut().item_spacing.x = 12.0;
                            let mut action = DialogAction::None;
                            if ui
                                .add_sized(egui::vec2(100.0, 40.0), egui::Button::new("キャンセル"))
                                .clicked()
                            {
                                action = DialogAction::Close;
                            }

                            if ui
                                .add_sized(egui::vec2(100.0, 40.0), egui::Button::new("追加する"))
                                .clicked()
                            {
                                let name = self.name.trim();
                                let url = self.url.trim();

                                if name.is_empty() {
                                    self.error = Some("レイヤー名を入力してください".to_owned());
                                } else if !(url.starts_with("https://")
                                    || url.starts_with("http://"))
                                {
                                    self.error = Some(
                                        "URLはhttp://またはhttps://で始めてください".to_owned(),
                                    );
                                } else if !["{z}", "{x}", "{y}"]
                                    .iter()
                                    .all(|placeholder| url.contains(placeholder))
                                {
                                    self.error =
                                        Some("URLに{z}、{x}、{y}をすべて含めてください".to_owned());
                                } else {
                                    let attribution = match (
                                        self.attribution.trim(),
                                        self.attribution_url.trim(),
                                    ) {
                                        ("", "") => None,
                                        (label, url) => Some((label.to_owned(), url.to_owned())),
                                    };
                                    action = DialogAction::Add(RasterTileLayer {
                                        id: String::new(),
                                        name: name.to_owned(),
                                        url: url.to_owned(),
                                        opacity: 1.0,
                                        visible: true,
                                        min_zoom: Some(0),
                                        max_zoom: Some(24),
                                        attribution,
                                    });
                                }
                            }
                            action
                        },
                    )
                    .inner
                })
            });

        match action
            .and_then(|response| response.inner)
            .map(|response| response.inner)
        {
            Some(DialogAction::Add(layer)) => {
                panel_visible_store.close_add_raster_tile_dialog();
                self.name.clear();
                self.url.clear();
                self.attribution.clear();
                self.attribution_url.clear();
                self.error = None;
                Some(layer)
            }
            Some(DialogAction::Close) => {
                panel_visible_store.close_add_raster_tile_dialog();
                None
            }
            Some(DialogAction::None) | None => {
                if !is_open {
                    panel_visible_store.close_add_raster_tile_dialog();
                }
                None
            }
        }
    }
}
