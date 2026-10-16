use domain::types::gis_engine_type::{MapXYZ, Screen};
use domain::types::map_layer_type::PictureLayer;
use eframe::egui;
use std::collections::HashMap;

/// PictureLayerの画像テクスチャを保持し、地図上へ描画する。
#[derive(Default)]
pub struct PictureLayerRenderer {
    picture_textures: HashMap<String, egui::TextureHandle>,
}

impl PictureLayerRenderer {
    /// 画像レイヤーを地図上の四隅に合わせて描画する。
    ///
    /// # Arguments
    /// * `ui` - 描画に使用するegui UI
    /// * `layer` - 画像データ、表示状態、地理座標の四隅を持つレイヤー
    /// * `map_xyz` - 現在の地図中心とズームレベル
    /// * `screen` - 地図表示領域のサイズ
    /// * `viewport` - 描画をクリップする画面領域
    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        layer: &PictureLayer,
        map_xyz: MapXYZ,
        screen: Screen,
        viewport: egui::Rect,
    ) {
        // 非表示または完全透明な画像は描画しない。
        if !layer.visible || layer.opacity <= 0.0 {
            return;
        }

        let Some(image_points) = Self::screen_corners(layer, map_xyz, screen, viewport) else {
            return;
        };

        // 表示範囲と交差しない画像は後続のテクスチャ処理を省略する。
        let bounds = calculate_bounds(&image_points);
        if !bounds.intersects(viewport) {
            return;
        }

        // 初回描画時に画像データをデコードし、egui用テクスチャとしてキャッシュする。
        if !self.picture_textures.contains_key(&layer.id) {
            let Ok(decoded_image) = image::load_from_memory(&layer.picture_data) else {
                return;
            };
            let rgba_image = decoded_image.to_rgba8();
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [rgba_image.width() as usize, rgba_image.height() as usize],
                rgba_image.as_raw(),
            );
            let texture = ui.ctx().load_texture(
                format!("picture-layer-{}", layer.id),
                color_image,
                egui::TextureOptions::LINEAR,
            );
            self.picture_textures.insert(layer.id.clone(), texture);
        }

        // テクスチャを四隅のメッシュへ貼り付けて描画する。
        if let Some(texture) = self.picture_textures.get(&layer.id) {
            // UV座標で元画像の四隅と地図上の四隅を対応付ける。
            let tint = egui::Color32::from_white_alpha(
                (layer.opacity.clamp(0.0, 1.0) * 255.0).round() as u8,
            );
            let uvs = [
                egui::pos2(0.0, 0.0),
                egui::pos2(1.0, 0.0),
                egui::pos2(1.0, 1.0),
                egui::pos2(0.0, 1.0),
            ];
            let mut mesh = egui::Mesh::with_texture(texture.id());
            mesh.vertices.extend((0..4).map(|index| egui::epaint::Vertex {
                pos: image_points[index],
                uv: uvs[index],
                color: tint,
            }));
            mesh.add_triangle(0, 1, 2);
            mesh.add_triangle(0, 2, 3);

            // 四角形を2つの三角形に分けてペインターへ渡す。
            ui.painter().add(egui::Shape::mesh(mesh));
        }
    }

    /// レイヤーのWGS84四隅を現在のビューポート上の画面座標へ変換する。
    pub fn screen_corners(
        layer: &PictureLayer,
        map_xyz: MapXYZ,
        screen: Screen,
        viewport: egui::Rect,
    ) -> Option<[egui::Pos2; 4]> {
        // Rectの各頂点を描画時と同じ順序で列挙する。
        let geographic_corners = [
            layer.picture_rect.top_left,
            layer.picture_rect.top_right,
            layer.picture_rect.bottom_right,
            layer.picture_rect.bottom_left,
        ];
        let mut points = [egui::Pos2::ZERO; 4];

        // 地図座標変換の結果にビューポート原点を加えて画面位置を得る。
        for (index, (longitude, latitude)) in geographic_corners.into_iter().enumerate() {
            let (x, y) = engine::lon_lat_to_pixel_xy(longitude, latitude, &map_xyz, screen).ok()?;
            points[index] = egui::pos2(viewport.min.x + x as f32, viewport.min.y + y as f32);
        }

        Some(points)
    }

    /// 削除済みレイヤーのテクスチャをキャッシュから取り除く。
    pub fn retain_layers(&mut self, layers: &[domain::types::map_layer_type::MapLayer]) {
        self.picture_textures.retain(|layer_id, _| {
            layers.iter().any(|layer| {
                matches!(layer, domain::types::map_layer_type::MapLayer::Picture(picture) if picture.id == *layer_id)
            })
        });
    }
}

/// 四隅をすべて含む最小の軸平行矩形を計算する。
pub fn calculate_bounds(points: &[egui::Pos2; 4]) -> egui::Rect {
    // 各座標の最小値と最大値を集めて矩形の両端を求める。
    let min = egui::pos2(
        points.iter().map(|point| point.x).fold(f32::INFINITY, f32::min),
        points.iter().map(|point| point.y).fold(f32::INFINITY, f32::min),
    );
    let max = egui::pos2(
        points
            .iter()
            .map(|point| point.x)
            .fold(f32::NEG_INFINITY, f32::max),
        points
            .iter()
            .map(|point| point.y)
            .fold(f32::NEG_INFINITY, f32::max),
    );
    egui::Rect::from_min_max(min, max)
}