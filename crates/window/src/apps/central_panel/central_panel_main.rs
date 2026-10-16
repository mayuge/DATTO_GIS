use crate::apps::store::map_engine_store::{MapEngineStore, MapEngineStoreTrait};
use crate::apps::store::map_layer_store::{MapLayerStore, MapLayerStoreTrait};
use super::picture_layer_editor::PictureLayerEditor;
use super::picture_layer_renderer::PictureLayerRenderer;
use super::raster_tile_renderer::RasterTileRenderer;
use domain::params::map_config::{MAP_MAX_ZOOM_LEVEL, MAP_MIN_ZOOM_LEVEL};
use domain::types::gis_engine_type::{MapXYZ, MoveXY, Screen};
use domain::types::map_layer_type::MapLayer;
use eframe::egui;
use engine::create_map_obj;

#[derive(Default)]
pub struct CentralPanelMain {
    picture_renderer: PictureLayerRenderer,
    picture_editor: PictureLayerEditor,
}

impl CentralPanelMain {
    /// 中央パネルを描画する。
    ///
    /// # Arguments
    /// * `ui` - 描画するUIコンテキスト
    /// * `ctx` - eguiのコンテキスト
    /// * `engine_store` - 地図エンジンの状態を管理するストア
    /// * `map_layer_store` - マップレイヤーの状態を管理するストア
    pub fn use_central_panel_main(
        &mut self,
        ui: &mut egui::Ui,
        engine_store: &mut MapEngineStore,
        map_layer_store: &mut MapLayerStore,
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

        // 地図エンジンに渡す入力を組み立てる。
        let mut engine_input = engine_store.get_map_engine_input();
        let previous_map_center = engine_store
            .get_map_object()
            .map(|map_object| map_object.center);
        let interaction_map_xyz = previous_map_center
            .map(|center| MapXYZ {
                latitude: center[1],
                longitude: center[0],
                zoom: engine_store.get_zoom(),
            })
            .unwrap_or(engine_input.map_xyz);
        
        // 画像レイヤーの編集状態を取得
        let editing_picture = self.picture_editor.use_picture_layer_editor(
            &response,
            &ctx,
            map_layer_store,
            interaction_map_xyz,
            engine_input.screen,
            viewport,
        );

        // 画像の編集中は地図へドラッグを渡さない。
        let movement = if editing_picture {
            MoveXY {
                move_x: 0.0,
                move_y: 0.0,
            }
        } else {
            Self::handle_pan_event(&response, &ctx)
        };

        // 前回の地図中心を使用してengine_inputを更新
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

        // リスト先頭を最前面にするため、固定ビューポート内でレイヤーを逆順に描画する。
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(viewport)
                .layout(egui::Layout::top_down(egui::Align::Min)),
            |ui| {
                ui.set_clip_rect(viewport);
                for map_layer in map_layer_store.get_all_layers().iter().rev() {
                    match map_layer {
                        MapLayer::RasterTile(layer) => {
                            RasterTileRenderer::render(
                                ui,
                                &map_obj,
                                layer,
                                engine_input.map_xyz.zoom,
                                viewport,
                            );
                        }
                        MapLayer::Picture(layer) => {
                            self.picture_renderer
                                .render(ui, layer, map_xyz, engine_input.screen, viewport);
                        }
                    }
                }
            }
        );
        self.picture_renderer
            .retain_layers(map_layer_store.get_all_layers());
        self.picture_editor.paint_selection(
            ui,
            map_layer_store.get_all_layers(),
            map_xyz,
            engine_input.screen,
            viewport,
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

}
