use domain::types::map_layer_type::RasterTileLayer;
use eframe::egui;

#[derive(Default)]
pub struct AddRasterTileDialog {
    is_open: bool,
    name: String,
    url: String,
    attribution: String,
    error: Option<String>,
}

impl AddRasterTileDialog {
    pub fn open(&mut self) {
        self.is_open = true;
        self.error = None;
    }

    pub fn show(&mut self, ctx: &egui::Context) -> Option<RasterTileLayer> {
        if !self.is_open {
            return None;
        }

        let mut added_layer = None;
        let mut should_close = false;

        egui::Window::new("ラスタータイルを追加")
            .open(&mut self.is_open)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("レイヤー名");
                ui.text_edit_singleline(&mut self.name);

                ui.label("タイルURL（{z}/{x}/{y} を含む）");
                ui.text_edit_singleline(&mut self.url);

                ui.label("出典（任意）");
                ui.text_edit_singleline(&mut self.attribution);

                if let Some(error) = &self.error {
                    ui.colored_label(ui.visuals().error_fg_color, error);
                }

                ui.horizontal(|ui| {
                    if ui.button("追加").clicked() {
                        let name = self.name.trim();
                        let url = self.url.trim();

                        if name.is_empty() {
                            self.error = Some("レイヤー名を入力してください".to_owned());
                        } else if !(url.starts_with("https://") || url.starts_with("http://")) {
                            self.error = Some("URLはhttp://またはhttps://で始めてください".to_owned());
                        } else if !["{z}", "{x}", "{y}"].iter().all(|placeholder| url.contains(placeholder)) {
                            self.error = Some("URLに{z}、{x}、{y}をすべて含めてください".to_owned());
                        } else {
                            added_layer = Some(RasterTileLayer {
                                id: String::new(),
                                name: name.to_owned(),
                                url: url.to_owned(),
                                opacity: 1.0,
                                visible: true,
                                attribution: (!self.attribution.trim().is_empty())
                                    .then(|| self.attribution.trim().to_owned()),
                            });
                            should_close = true;
                        }
                    }

                    if ui.button("キャンセル").clicked() {
                        should_close = true;
                    }
                });
            });

        if should_close {
            self.is_open = false;
        }

        if added_layer.is_some() {
            self.name.clear();
            self.url.clear();
            self.attribution.clear();
            self.error = None;
        }

        added_layer
    }
}