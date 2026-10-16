use domain::params::map_config::MAP_TILE_SIZE;
use domain::types::gis_engine_type::MapObject;
use domain::types::map_layer_type::RasterTileLayer;
use eframe::egui;

/// ラスタータイルレイヤーの描画を担当する。
pub struct RasterTileRenderer;

impl RasterTileRenderer {
    /// 地図オブジェクト内のタイルをビューポートへ描画する。
    ///
    /// # Arguments
    /// * `ui` - 描画に使用するegui UI
    /// * `map_obj` - タイル座標と地図上のオフセットを持つ地図オブジェクト
    /// * `layer` - 表示状態やURL、不透明度を持つレイヤー
    /// * `zoom` - 現在のズームレベル
    /// * `viewport` - タイルを描画する画面領域
    pub fn render(
        ui: &mut egui::Ui,
        map_obj: &MapObject,
        layer: &RasterTileLayer,
        zoom: u32,
        viewport: egui::Rect,
    ) {
        // 描画対象のタイルがない場合や、レイヤーが非表示の場合は処理しない。
        if map_obj.tiles.is_empty() || !layer.visible || layer.opacity <= 0.0 {
            return;
        }

        // レイヤーに設定されたズーム範囲外では描画しない。
        if layer
            .min_zoom
            .is_some_and(|min_zoom| zoom < u32::from(min_zoom))
            || layer
                .max_zoom
                .is_some_and(|max_zoom| zoom > u32::from(max_zoom))
        {
            return;
        }

        // 最初のタイルを基準に、各タイルの相対位置を計算する。
        let first_tile = &map_obj.tiles[0];

        // レイヤー不透明度をeguiのアルファ値へ変換する。
        let alpha = (layer.opacity.clamp(0.0, 1.0) * 255.0).round() as u8;

        // ビューポート内にあるタイルだけURLを組み立てて描画する。
        for tile in &map_obj.tiles {
            let x = viewport.min.x
                + map_obj.offset[0] as f32
                + (tile.x as f32 - first_tile.x as f32) * MAP_TILE_SIZE;
            let y = viewport.min.y
                + map_obj.offset[1] as f32
                + (tile.y as f32 - first_tile.y as f32) * MAP_TILE_SIZE;
            let rect = egui::Rect::from_min_size(
                egui::pos2(x, y),
                egui::vec2(MAP_TILE_SIZE, MAP_TILE_SIZE),
            );

            // 画面領域と交差しないタイルは描画を省略する。
            if !rect.intersects(viewport) {
                continue;
            }

            // URLテンプレートのXYZプレースホルダーを実際の座標へ置き換える。
            let url = layer
                .url
                .replace("{z}", &tile.z.to_string())
                .replace("{x}", &tile.x.to_string())
                .replace("{y}", &tile.y.to_string());

            // 計算した位置へ不透明度を適用してタイル画像を描画する。
            egui::Image::new(url)
                .tint(egui::Color32::from_white_alpha(alpha))
                .paint_at(ui, rect);
        }
    }
}