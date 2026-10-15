use crate::apps::store::map_engine_store::{MapEngineStore, MapEngineStoreTrait};
use crate::apps::store::raster_tile_layer_store::{
    RasterTileLayerStore, RasterTileLayerStoreTrait,
};
use domain::params::map_config::{MAP_MAX_ZOOM_LEVEL, MAP_MIN_ZOOM_LEVEL, MAP_TILE_SIZE};
use domain::types::gis_engine_type::{MapObject, MapXYZ, MoveXY, Screen};
use eframe::egui;
use engine::create_map_obj;

pub struct CentralPanelMain; 

impl CentralPanelMain {
    /// 中央パネルを描画する。
    ///
    /// # Arguments
    /// * `ui` - 描画するUIコンテキスト
    /// * `ctx` - eguiのコンテキスト
    /// * `engine_store` - 地図エンジンの状態を管理するストア
    /// * `raster_tile_layer_store` - ラスタータイルレイヤーの状態を管理するストア
    pub fn use_central_panel_main(
        &mut self,
        ui: &mut egui::Ui,
        engine_store: &mut MapEngineStore,
        raster_tile_layer_store: &RasterTileLayerStore,
    ) {
        let ctx = ui.ctx().clone();

        // サイドパネルを除いた中央パネルのサイズを取得
        let viewport = ui.available_rect_before_wrap();
        let available_size = viewport.size();

        if available_size.x <= 0.0 || available_size.y <= 0.0 {
            return;
        }

        // 描画領域のサイズを設定
        engine_store.set_screen(Screen {
            width: available_size.x as f64,
            height: available_size.y as f64,
        });

        // 地図領域の入力を取得
        let response = ui.interact(
            viewport,
            ui.id().with("map_viewport"),
            egui::Sense::click_and_drag(),
        );

        // ズーム処理
        Self::handle_zoom_event(&response, &ctx, engine_store);

        // パン移動量はフレームごとに適用し、前回の地図中心を基準にする。
        let movement = Self::handle_pan_event(&response, &ctx);

        // 地図エンジンに渡す入力を組み立てる。
        let mut engine_input = engine_store.get_map_engine_input();
        let previous_map_center = engine_store
            .get_map_object()
            .map(|map_object| map_object.center);
        if let Some(center) = previous_map_center {
            engine_input.map_xyz = MapXYZ {
                latitude: center[1],
                longitude: center[0],
                zoom: engine_store.get_zoom(),
            };
        }
        engine_input.movement = movement;

        let map_obj = match create_map_obj(&engine_input) {
            Ok(map_obj) => map_obj,
            Err(error) => {
                eprintln!("GISエンジンのエラーです: {error}");
                return;
            }
        };
        let map_xyz = MapXYZ {
            latitude: map_obj.center[1],
            longitude: map_obj.center[0],
            zoom: engine_input.map_xyz.zoom,
        };

        // ラスタータイル描画
        Self::render_raster_tiles(
            ui,
            &map_obj,
            raster_tile_layer_store,
            engine_input.map_xyz.zoom,
        );
        engine_store.set_xyz(map_xyz);
        engine_store.set_map_object(map_obj);
    }

    /// マウスホイールによるズーム操作を処理する。
    ///
    /// # Arguments
    /// * `ui` - 描画するUIコンテキスト
    /// * `ctx` - eguiのコンテキスト
    /// * `engine_store` - 地図エンジンの状態を管理する
    fn handle_zoom_event(
        response: &egui::Response,
        ctx: &egui::Context,
        engine_store: &mut MapEngineStore,
    ) {
        if !response.hovered() {
            return;
        }

        let scroll_directions: Vec<f32> = ctx.input(|input| {
            input
                .events
                .iter()
                .filter_map(|event| {
                    if let egui::Event::MouseWheel { delta, .. } = event {
                        (delta.y != 0.0).then_some(delta.y.signum())
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

        for direction in scroll_directions {
            zoom = if direction > 0.0 {
                //ズームレベルが最大値でない場合、ズームレベルを1つ増やす
                zoom.saturating_add(1).min(MAP_MAX_ZOOM_LEVEL)
            } else {
                //ズームレベルが最小値でない場合、ズームレベルを1つ減らす
                zoom.saturating_sub(1).max(MAP_MIN_ZOOM_LEVEL)
            };
        }

        engine_store.set_zoom(zoom);
        ctx.request_repaint();
    }

    /// マウスドラッグによるフレーム単位の移動量を返す。
    fn handle_pan_event(response: &egui::Response, ctx: &egui::Context) -> MoveXY {
        if response.dragged() || response.drag_stopped() {
            let delta = ctx.input(|input| input.pointer.delta());
            ctx.request_repaint();
            MoveXY {
                move_x: f64::from(delta.x),
                move_y: f64::from(delta.y),
            }
        } else {
            MoveXY {
                move_x: 0.0,
                move_y: 0.0,
            }
        }
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
        zoom: u32,
    ) {
        // タイルがない場合は描画しない
        if map_obj.tiles.is_empty() {
            return;
        }

        let viewport = ui.available_rect_before_wrap();
        let origin = viewport.min;

        // 最初のタイルを基準に配置
        let first_tile = &map_obj.tiles[0];

        let offset_x = map_obj.offset[0] as f32;
        let offset_y = map_obj.offset[1] as f32;

        let layers = raster_tile_layer_store.get_all_raster_tile_layers();

        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(viewport)
                .layout(egui::Layout::top_down(egui::Align::Min)),
            |ui| {
                ui.set_clip_rect(viewport);

                // 下のレイヤーから描画
                for layer in layers.iter().rev() {
                    //レイヤーがオフまたは不透明度が0以下の場合は描画しない
                    if !layer.visible || layer.opacity <= 0.0 {
                        continue;
                    }
                    // 最大・最小ズーム範囲ではない場合は描画しない
                    if layer
                        .min_zoom
                        .is_some_and(|min_zoom| zoom < u32::from(min_zoom))
                        || layer
                            .max_zoom
                            .is_some_and(|max_zoom| zoom > u32::from(max_zoom))
                    {
                        continue;
                    }

                    // レイヤーの不透明度をアルファ値に変換
                    let alpha = (layer.opacity.clamp(0.0, 1.0) * 255.0).round() as u8;

                    for tile in &map_obj.tiles {
                        let x = origin.x
                            + offset_x
                            + (tile.x as f32 - first_tile.x as f32) * MAP_TILE_SIZE;
                        let y = origin.y
                            + offset_y
                            + (tile.y as f32 - first_tile.y as f32) * MAP_TILE_SIZE;

                        let rect = egui::Rect::from_min_size(
                            egui::pos2(x, y),
                            egui::vec2(MAP_TILE_SIZE, MAP_TILE_SIZE),
                        );
                        // ビューポートに含まれていないタイルは描画しない
                        if !rect.intersects(viewport) {
                            continue;
                        }

                        let url = layer
                            .url
                            .replace("{z}", &tile.z.to_string())
                            .replace("{x}", &tile.x.to_string())
                            .replace("{y}", &tile.y.to_string());

                        ui.put(
                            rect,
                            egui::Image::new(url).tint(egui::Color32::from_white_alpha(alpha)),
                        );
                    }
                }
            },
        );
    }
}
