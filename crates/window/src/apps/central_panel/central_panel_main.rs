use crate::apps::store::map_engine_store::{MapEngineStore, MapEngineStoreTrait};
use crate::apps::store::raster_tile_layer_store::{
    RasterTileLayerStore, RasterTileLayerStoreTrait,
};
use domain::params::map_config::MAP_TILE_SIZE;
use domain::types::gis_engine_type::{MapObject, Screen};
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
    /// 中央パネルを描画する。
    ///
    /// # Arguments
    /// * `ui` - 描画するUIコンテキスト
    /// * `ctx` - eguiのコンテキスト
    /// * `engine_store` - 地図エンジンの状態を管理するストア
    /// * `raster_tile_layer_store` - ラスタータイルレイヤーの状態を管理するストア
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

        // 描画領域のサイズを設定
        engine_store.set_screen(Screen {
            width: available_size.x as f64,
            height: available_size.y as f64,
        });

        // ズーム処理
        Self::handle_zoom_event(ui, ctx, engine_store);

        // 地図エンジン
        let map_obj = match create_map_obj(engine_store.get_all()) {
            Ok(map_obj) => map_obj,
            Err(error) => {
                eprintln!("GISエンジンのエラーです: {error}");
                return;
            }
        };

        // ラスタータイル描画
        Self::render_raster_tiles(ui, &map_obj, raster_tile_layer_store);
    }

    /// マウスホイールによるズーム操作を処理する。
    ///
    /// # Arguments
    /// * `ui` - 描画するUIコンテキスト
    /// * `ctx` - eguiのコンテキスト
    /// * `engine_store` - 地図エンジンの状態を管理する
    /// マウスホイールイベントによるズーム操作を処理する。
    fn handle_zoom_event(
        ui: &mut egui::Ui,
        ctx: &egui::Context,
        engine_store: &mut MapEngineStore,
    ) {
        let viewport = ui.available_rect_before_wrap();

        let response = ui.interact(viewport, ui.id().with("map_viewport"), egui::Sense::hover());

        // マウスカーソルが地図上にない場合は処理しない。
        if !response.hovered() {
            return;
        }

        // 今フレームに発生したマウスホイールイベントを取得する。
        let scroll_directions: Vec<f32> = ctx.input(|input| {
            input
                .events
                .iter()
                .filter_map(|event| {
                    if let egui::Event::MouseWheel { delta, .. } = event {
                        if delta.y != 0.0 {
                            Some(delta.y.signum())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect()
        });

        if scroll_directions.is_empty() {
            return;
        }

        let mut zoom = engine_store.get_zoom();

        // ホイールイベントごとにズームを1段階変更する。
        for direction in scroll_directions {
            zoom = if direction > 0.0 {
                // direction > 0.0 の場合、ズームイン
                zoom.saturating_add(1).min(22)
            } else {
                // direction < 0.0 の場合、ズームアウト
                zoom.saturating_sub(1)
            };
        }

        engine_store.set_zoom(zoom);
        ctx.request_repaint();
    }

    /// ラスタータイルを描画する。
    ///
    /// # Arguments
    /// * `ui` - 描画するUIコンテキスト
    /// * `map_obj` - 地図オブジェクト
    /// * `raster_tile_layer_store` - ラスタータイルレイヤーの状態を管理するストア
    fn render_raster_tiles(
        ui: &mut egui::Ui,
        map_obj: &MapObject,
        raster_tile_layer_store: &RasterTileLayerStore,
    ) {
        // タイルがない場合は描画しない
        if map_obj.tiles.is_empty() {
            return;
        }

        // 描画領域
        let available_size = ui.available_size();
        let origin = ui.min_rect().min;
        let viewport = egui::Rect::from_min_size(origin, available_size);

        // 最初のタイルを基準に配置
        let first_tile = &map_obj.tiles[0];

        let offset_x = map_obj.offset[0] as f32;
        let offset_y = map_obj.offset[1] as f32;

        let layers = raster_tile_layer_store.get_all();

        // 下のレイヤーから描画
        for layer in layers.iter().rev() {
            if !layer.visible || layer.opacity <= 0.0 {
                continue;
            }

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

                // タイルURLを生成
                let url = layer
                    .url
                    .replace("{z}", &tile.z.to_string())
                    .replace("{x}", &tile.x.to_string())
                    .replace("{y}", &tile.y.to_string());

                ui.put(rect, egui::Image::new(url));
            }
        }
    }
}
