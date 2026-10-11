use crate::apps::central_panel::central_panel_main::CentralPanelMain;
use crate::apps::store::map_engine_store::MapEngineStore;
use crate::apps::store::raster_tile_layer_store::RasterTileLayerStore;
use eframe::egui;
use std::sync::Arc;

/// アプリケーションを起動する。
pub fn create_app() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("DattoGIS")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "DattoGIS",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(App::new()))
        }),
    )
    .expect("DattoGIS の起動に失敗しました");
}

/// フォントを登録する。
fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Noto Sans JP
    fonts.font_data.insert(
        "noto_sans_jp".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/NotoSansJP-Regular.ttf"
        ))),
    );

    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "noto_sans_jp".to_owned());

    // Material Icons
    fonts.font_data.insert(
        "material_icons".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/MaterialIcons-Regular.ttf"
        ))),
    );

    fonts.families.insert(
        egui::FontFamily::Name("material_icons".into()),
        vec!["material_icons".to_owned()],
    );

    ctx.set_fonts(fonts);
}

/// アプリケーションの状態と画面を管理する。
struct App {
    central_panel: CentralPanelMain,
    engine_store: MapEngineStore,
    layer_store: RasterTileLayerStore,
}

impl App {
    fn new() -> Self {
        Self {
            central_panel: CentralPanelMain::default(),
            engine_store: MapEngineStore::default(),
            layer_store: RasterTileLayerStore::default(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        self.central_panel
            .show(ui, &ctx, &mut self.engine_store, &self.layer_store);
    }
}
