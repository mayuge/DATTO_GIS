
use eframe::egui;
use domain::types::gis_engine_type::{MapXYZ, Screen};
use domain::types::map_layer_type::PictureLayer;
use engine::{SetPictureLayerInput, set_picture_layer};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct AddImageFileDialog {
    is_open: bool,
    image_name: String,
    image_path: String,
    image_data: Option<Vec<u8>>,
    error: Option<String>,
    pending_image_data: Arc<Mutex<Option<Result<(String, String, Vec<u8>), String>>>>,
}

impl AddImageFileDialog {
    /// ダイアログを開き、新しい画像の入力を始める。
    pub fn open(&mut self) {
        self.is_open = true;
        self.image_name.clear();
        self.image_path.clear();
        self.image_data = None;
        self.error = None;
    }

    /// 画像追加ダイアログを表示し、ユーザーの操作を処理する。
    /// 追加が確定した場合は、画像データを含むレイヤーを呼び出し元へ返す。
    pub fn use_add_image_file_dialog(
        &mut self,
        ctx: &egui::Context,
        map_xyz: MapXYZ,
        map_screen_size: Screen,
    ) -> Option<PictureLayer> {
        if !self.is_open {
            return None;
        }

        let mut is_open = self.is_open;
        let mut dropped_image = self.pending_image_data.lock().unwrap().take();
        let mut added_layer = None;
        let mut should_close = false;

        // ブラウザーでは画像の読み込みが非同期なので、完了結果を次のフレームで受け取る。
        if let Some(result) = dropped_image.take() {
            match result {
                Ok(image) => dropped_image = Some(Ok(image)),
                Err(error) => self.error = Some(error),
            }
        }

        egui::Window::new("画像を追加")
            .open(&mut is_open)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("画像をドロップして、名前とパスを確認してください。");

                // ドロップ領域はプレビュー用の小さな目印とし、ファイル受信はegui入力から行う。
                let has_hovered_file = ctx.input(|input| !input.raw.hovered_files.is_empty());
                let drop_area = egui::Frame::group(ui.style())
                    .show(ui, |ui| {
                        ui.set_min_size(egui::vec2(320.0, 64.0));
                    })
                    .response;

                let drop_label = if has_hovered_file && drop_area.hovered() {
                    "ここにドロップして画像を読み込み"
                } else {
                    "画像ファイルをここにドロップ"
                };
                ui.painter().text(
                    drop_area.rect.center(),
                    egui::Align2::CENTER_CENTER,
                    drop_label,
                    egui::TextStyle::Body.resolve(ui.style()),
                    ui.visuals().text_color(),
                );

                // eguiが受け取ったファイル情報を取り出し、画像の内容を読み込む。
                let dropped_files = ctx.input(|input| input.raw.dropped_files.clone());

                #[cfg(not(target_arch = "wasm32"))]
                for file in dropped_files {
                    match Self::image_info(file.as_ref()).and_then(|(name, path)| {
                        Self::read_image_file(file.as_ref())
                            .map(|bytes| (name, path, bytes))
                    }) {
                        Ok(image) => {
                            dropped_image = Some(Ok(image));
                            break;
                        }
                        Err(error) => dropped_image = Some(Err(error)),
                    }
                }

                #[cfg(target_arch = "wasm32")]
                if let Some(file) = dropped_files.into_iter().next() {
                    match Self::image_info(file.as_ref()) {
                        Ok((name, path)) => {
                            let pending_image_data = Arc::clone(&self.pending_image_data);
                            let ctx = ctx.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                let result = file
                                    .bytes_async()
                                    .await
                                    .map(|bytes| (name, path, bytes));
                                *pending_image_data.lock().unwrap() = Some(result);
                                ctx.request_repaint();
                            });
                        }
                        Err(error) => dropped_image = Some(Err(error)),
                    }
                }

                // ドロップしたファイル名とパスを編集できるようにする。
                ui.label("名前");
                ui.text_edit_singleline(&mut self.image_name);
                ui.label("パス（ブラウザーではファイル名のみ）");
                ui.text_edit_singleline(&mut self.image_path);

                if let Some(error) = &self.error {
                    ui.colored_label(ui.visuals().error_fg_color, error);
                }

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("キャンセル").clicked() {
                        should_close = true;
                        self.image_name.clear();
                        self.image_path.clear();
                        self.image_data = None;
                        self.error = None;
                    }

                    if ui.button("追加する").clicked() {
                        // 画像が未選択、または名前が空の状態では登録しない。
                        let name = self.image_name.trim();
                        if name.is_empty() {
                            self.error = Some("レイヤー名を入力してください".to_owned());
                        } else if let Some(picture_data) = &self.image_data {
                            match image::load_from_memory(picture_data) {
                                Ok(image) => {
                                    let picture_size = Screen {
                                        width: f64::from(image.width()),
                                        height: f64::from(image.height()),
                                    };
                                    match set_picture_layer(SetPictureLayerInput {
                                        map_xyz,
                                        map_screen_size,
                                        picture_size,
                                    }) {
                                        Ok(picture_rect) => {
                                            added_layer = Some(PictureLayer {
                                                id: String::new(),
                                                name: name.to_owned(),
                                                opacity: 1.0,
                                                visible: true,
                                                picture_size,
                                                picture_data: picture_data.clone(),
                                                picture_path: self.image_path.clone(),
                                                picture_rect,
                                            });
                                            should_close = true;
                                        }
                                        Err(error) => {
                                            self.error = Some(format!(
                                                "画像の配置座標を計算できません: {error}"
                                            ));
                                        }
                                    }
                                }
                                Err(error) => {
                                    self.error = Some(format!("画像を読み込めません: {error}"));
                                }
                            }
                        } else {
                            self.error = Some("画像ファイルをドロップしてください".to_owned());
                        }
                    }
                });
            });

        if should_close {
            is_open = false;
        }
        self.is_open = is_open;
        // ドロップ結果を入力欄と画像データに反映する。
        if let Some(result) = dropped_image {
            match result {
                Ok((name, path, bytes)) => {
                    self.image_name = name;
                    self.image_path = path;
                    self.image_data = Some(bytes);
                    self.error = None;
                }
                Err(error) => self.error = Some(error),
            }
        }

        // ウィンドウの閉じるボタンを押した場合も、キャンセルと同じ状態に戻す。
        if !self.is_open && added_layer.is_none() {
            self.image_name.clear();
            self.image_path.clear();
            self.image_data = None;
        }

        added_layer
    }

    /// ファイル名と、実行環境から取得できるパスを返す。
    /// WASMではブラウザーの制約により、パスはファイル名だけになる。
    fn image_info(file: &dyn egui::DroppedFile) -> Result<(String, String), String> {
        let path = file.path();
        let extension = path
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        if !matches!(extension.as_str(), "png" | "jpg" | "jpeg" | "tif" | "tiff") {
            return Err("PNG、JPEG、TIFF形式の画像をドロップしてください".to_owned());
        }

        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .map(str::to_owned)
            .ok_or_else(|| "画像ファイル名を取得できませんでした".to_owned())?;

        Ok((name, path.to_string_lossy().into_owned()))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn read_image_file(file: &dyn egui::DroppedFile) -> Result<Vec<u8>, String> {
        file.bytes()
    }
}
