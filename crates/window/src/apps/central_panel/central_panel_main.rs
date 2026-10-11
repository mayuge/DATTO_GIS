use crate::apps::store::map_engine_store::{MapEngineStore, MapEngineStoreTrait};
use crate::apps::store::raster_tile_layer_store::{
    RasterTileLayerStore, RasterTileLayerStoreTrait,
};
use domain::params::map_config::MAP_TILE_SIZE;
use domain::types::gis_engine_type::Screen;
use eframe::egui;
use engine::create_map_obj;

pub struct CentralPanelMain {
    pub map_engine_store: MapEngineStore,
    pub raster_tile_layer_store: RasterTileLayerStore,
}

impl Default for CentralPanelMain {
    fn default() -> Self {
        Self {
            map_engine_store: MapEngineStore::default(),
            raster_tile_layer_store: RasterTileLayerStore::default(),
        }
    }
}

impl CentralPanelMain {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        engine_store: &mut MapEngineStore,
        raster_tile_layer_store: &RasterTileLayerStore,
    ) {
        // 描画領域
        let available_size = ui.available_size();

        if available_size.x <= 0.0 || available_size.y <= 0.0 {
            return;
        }

        engine_store.set_screen(Screen {
            width: available_size.x as f64,
            height: available_size.y as f64,
        });

        // 地図エンジン
        let map_obj = match create_map_obj(engine_store.get_all()) {
            Ok(map_obj) => map_obj,
            Err(error) => {
                eprintln!("GISエンジンのエラーです: {error}");
                return;
            }
        };

        if map_obj.tiles.is_empty() {
            return;
        }

        // 描画領域の左上
        let origin = ui.min_rect().min;

        // 表示範囲
        let viewport = egui::Rect::from_min_size(origin, available_size);

        // 最初のタイルを基準に配置する
        // engine 側で (y, x) 順にソートされていることが前提
        let first_tile = &map_obj.tiles[0];

        let offset_x = map_obj.offset[0] as f32;
        let offset_y = map_obj.offset[1] as f32;

        let layers = raster_tile_layer_store.get_all();

        // 下のレイヤーから描画する
        for layer in layers.iter().rev() {
            for tile in &map_obj.tiles {
                let x = origin.x + offset_x + (tile.x as f32 - first_tile.x as f32) * MAP_TILE_SIZE;

                let y = origin.y + offset_y + (tile.y as f32 - first_tile.y as f32) * MAP_TILE_SIZE;

                let rect = egui::Rect::from_min_size(
                    egui::pos2(x, y),
                    egui::vec2(MAP_TILE_SIZE, MAP_TILE_SIZE),
                );

                // 画面外のタイルは描画しない
                if !rect.intersects(viewport) {
                    continue;
                }

                let url = layer
                    .url
                    .replace("{z}", &tile.z.to_string())
                    .replace("{x}", &tile.x.to_string())
                    .replace("{y}", &tile.y.to_string());

                ui.put(rect, egui::Image::new(url));
            }
        }

        // 再描画が必要な場合に備えてコンテキストを受け取る
        let _ = ctx;
    }
}
