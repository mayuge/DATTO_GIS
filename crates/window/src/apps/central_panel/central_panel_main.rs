use crate::apps::store::map_engine_store::{MapEngineStore, MapEngineStoreTrait};
use crate::apps::store::map_layer_store::{MapLayerStore, MapLayerStoreTrait};
use domain::Screen;
use eframe::egui;
use engine::create_map_obj;

pub struct CentralPanelMain {
    pub map_engine_store: MapEngineStore,
    pub map_layer_store: MapLayerStore,
}

trait CentralPanelMainTrait {
    fn show(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        engine_store: &MapEngineStore,
        layer_store: &MapLayerStore,
    );
}

impl Default for CentralPanelMain {
    fn default() -> Self {
        Self {
            map_engine_store: MapEngineStore::default(),
            map_layer_store: MapLayerStore::default(),
        }
    }
}

impl CentralPanelMain {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        engine_store: &mut MapEngineStore,
        layer_store: &MapLayerStore,
    ) {
        // 地図を描画できる領域のサイズを取得
        let available_size = ui.available_size();

        // 画面の幅と高さをストアに設定
        engine_store.set_screen(Screen {
            width: available_size.x as f64,
            height: available_size.y as f64,
        });

        //engineを呼び出す
        let map_obj = create_map_obj(engine_store.get_all());
    }
}
