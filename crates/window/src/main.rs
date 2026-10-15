mod apps;
#[cfg(not(target_arch = "wasm32"))]
use apps::app::create_app;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// アプリケーションを起動するエントリーポイント。
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    create_app();
}

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
fn start() {
    wasm_bindgen_futures::spawn_local(async {
        let result = async {
            let window = web_sys::window().ok_or("ブラウザのウィンドウを取得できません")?;
            let document = window
                .document()
                .ok_or("ブラウザのドキュメントを取得できません")?;
            let canvas = document
                .get_element_by_id("datto-gis-canvas")
                .ok_or("地図キャンバスが見つかりません")?
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| "地図キャンバスの型が不正です")?;

            eframe::WebRunner::new()
                .start(
                    canvas,
                    eframe::WebOptions::default(),
                    Box::new(|cc| {
                        cc.egui_ctx.set_theme(eframe::egui::ThemePreference::Dark);
                        egui_extras::install_image_loaders(&cc.egui_ctx);
                        apps::app::setup_fonts(&cc.egui_ctx);
                        Ok(Box::new(apps::app::create_app_state()))
                    }),
                )
                .await
                .map_err(|error| format!("WASM アプリの起動に失敗しました: {error:?}"))
        }
        .await;

        if let Err(error) = result {
            web_sys::console::error_1(&error.into());
        }
    });
}
