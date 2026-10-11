use crate::apps::store::map_engine_store::{MapEngineStore, MapEngineStoreTrait};
use crate::apps::store::map_layer_store::{RasterTileLayerStore, RasterTileLayerStoreTrait};
use domain::Screen;
use eframe::egui;
use engine::create_map_obj;

pub struct CentralPanelMain {
    pub map_engine_store: MapEngineStore,
    pub map_layer_store: RasterTileLayerStore,
}

trait CentralPanelMainTrait {
    fn show(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        engine_store: &MapEngineStore,
        layer_store: &RasterTileLayerStore,
    );
}

impl Default for CentralPanelMain {
    fn default() -> Self {
        Self {
            map_engine_store: MapEngineStore::default(),
            map_layer_store: RasterTileLayerStore::default(),
        }
    }
}

impl CentralPanelMain {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        engine_store: &mut MapEngineStore,
        layer_store: &RasterTileLayerStore,
    ) {
        // 地図を描画できる領域のサイズを取得
        let available_size = ui.available_size();

        // 画面の幅と高さをストアに設定
        engine_store.set_screen(Screen {
            width: available_size.x as f64,
            height: available_size.y as f64,
        });

        //engineを呼び出す
        // エンジンを呼び出す
        let map_obj = match create_map_obj(engine_store.get_all()) {
            Ok(map_obj) => map_obj,
            Err(error) => {
                eprintln!("GISエンジンのエラーです: {error}");
                return;
            }
        };

        // TileZXY を /z/x/y 形式に変換
        let tiles: Vec<String> = map_obj
            .tiles
            .iter()
            .map(|tile| format!("/{}/{}/{}", tile.z, tile.x, tile.y))
            .collect();

        println!("Tiles to fetch: {:?}", tiles);

        // レイヤーを呼び出す
        let layers = layer_store.get_all();
    }
}
