use crate::apps::store::map_layer_store::{MapLayerStore, MapLayerStoreTrait};
use domain::types::gis_engine_type::{MapXYZ, Screen};
use domain::types::map_layer_type::{MapLayer, Rect};
use eframe::egui;

use super::picture_layer_renderer::PictureLayerRenderer;

const HANDLE_RADIUS: f32 = 4.0;
const HANDLE_HIT_RADIUS: f32 = 12.0;
const ROTATION_HANDLE_OFFSET: f32 = 30.0;
const ROTATION_HANDLE_RADIUS: f32 = 12.0;
const ROTATION_HANDLE_HIT_RADIUS: f32 = 19.0;
const SCALE_HANDLE_OFFSET: f32 = 24.0;
const SCALE_HANDLE_RADIUS: f32 = 10.0;
const SCALE_HANDLE_HIT_RADIUS: f32 = 16.0;

/// PictureLayerの選択状態と現在のドラッグ操作を管理する。
#[derive(Default)]
pub struct PictureLayerEditor {
    selected_layer_id: Option<String>,
    drag: Option<PictureLayerDrag>,
}

/// ドラッグ開始時の形状と操作を保持する。
struct PictureLayerDrag {
    layer_id: String,
    mode: DragMode,
    start_pointer: egui::Pos2,
    initial_corners: [egui::Pos2; 4],
}

/// ドラッグ対象に応じた画像の変形方法。
#[derive(Clone, Copy)]
enum DragMode {
    /// 指定された頂点だけを移動する。
    Corner(usize),
    /// 四隅を同じ量だけ移動する。
    Translate,
    /// 画像の中心を軸に回転する。
    Rotate,
    /// 中心を保ったまま四隅を同じ倍率で拡大・縮小する。
    Scale,
}

impl PictureLayerEditor {
    /// ビューポートのポインター入力を画像編集または地図操作へ振り分ける。
    ///
    /// 戻り値は、このフレームのドラッグを画像編集が処理したかどうか。
    pub fn use_picture_layer_editor(
        &mut self,
        response: &egui::Response,
        ctx: &egui::Context,
        map_layer_store: &mut MapLayerStore,
        map_xyz: MapXYZ,
        screen: Screen,
        viewport: egui::Rect,
    ) -> bool {
        let was_dragging = self.drag.is_some();

        // ドラッグ開始位置から編集対象と操作種別を特定する。
        if response.drag_started() && self.drag.is_none() {
            let start_pointer = ctx.input(|input| {
                input
                    .pointer
                    .press_origin()
                    .or_else(|| input.pointer.interact_pos())
            });
            if let Some(start_pointer) = start_pointer {
                if let Some((layer_id, mode, initial_corners)) = hit_test(
                    map_layer_store.get_all_layers(),
                    self.selected_layer_id.as_deref(),
                    start_pointer,
                    map_xyz,
                    screen,
                    viewport,
                ) {
                    self.selected_layer_id = Some(layer_id.clone());
                    self.drag = Some(PictureLayerDrag {
                        layer_id,
                        mode,
                        start_pointer,
                        initial_corners,
                    });
                }
            }
        }

        // ドラッグ中は開始時の形状を基準に四隅を更新する。
        if let Some(drag) = &self.drag {
            if response.dragged() || response.drag_stopped() {
                if let Some(pointer) = ctx.input(|input| input.pointer.interact_pos()) {
                    if let Some(picture_rect) = dragged_rect(
                        drag,
                        pointer,
                        map_xyz,
                        screen,
                        viewport,
                    ) {
                        map_layer_store.update_picture_rect(&drag.layer_id, picture_rect);
                        ctx.request_repaint();
                    }
                }
            }
        }

        // ドラッグ終了後に状態を解放し、通常クリックなら選択対象を更新する。
        let handled = was_dragging || self.drag.is_some();
        if response.drag_stopped() {
            self.drag = None;
        }

        if response.clicked() && !handled {
            let pointer = ctx.input(|input| input.pointer.interact_pos());
            self.selected_layer_id = pointer.and_then(|pointer| {
                hit_test(
                    map_layer_store.get_all_layers(),
                    self.selected_layer_id.as_deref(),
                    pointer,
                    map_xyz,
                    screen,
                    viewport,
                )
                .map(|(layer_id, _, _)| layer_id)
            });
            ctx.request_repaint();
        }

        handled
    }

    /// 選択中の画像の輪郭、角ハンドル、回転ハンドルを描画する。
    pub fn paint_selection(
        &self,
        ui: &egui::Ui,
        layers: &[MapLayer],
        map_xyz: MapXYZ,
        screen: Screen,
        viewport: egui::Rect,
    ) {
        // 選択中かつ表示中の画像を探す。
        let Some(layer) = self.selected_layer_id.as_deref().and_then(|selected_id| {
            layers.iter().find_map(|layer| match layer {
                MapLayer::Picture(picture)
                    if picture.id == selected_id && picture.visible && picture.opacity > 0.0 =>
                {
                    Some(picture)
                }
                _ => None,
            })
        }) else {
            return;
        };

        // 現在の地図表示に合わせて四隅を画面座標へ変換する。
        let Some(corners) =
            PictureLayerRenderer::screen_corners(layer, map_xyz, screen, viewport)
        else {
            return;
        };

        let painter = ui.painter().with_clip_rect(viewport);
        let outline = egui::Color32::from_hex("#23CDBE").expect("valid selection color");

        // 四隅を結んで画像の選択枠を表示する。
        for index in 0..4 {
            painter.line_segment(
                [corners[index], corners[(index + 1) % 4]],
                egui::Stroke::new(1.5, outline),
            );
        }

        // 上辺の外側に回転ハンドルを表示する。
        if let Some(rotation_handle) = rotation_handle(&corners) {
            painter.line_segment(
                [midpoint(corners[0], corners[1]), rotation_handle],
                egui::Stroke::new(1.0, outline),
            );
            painter.circle_filled(rotation_handle, ROTATION_HANDLE_RADIUS, outline);
            painter.circle_stroke(
                rotation_handle,
                ROTATION_HANDLE_RADIUS,
                egui::Stroke::new(1.5, egui::Color32::WHITE),
            );
            painter.text(
                rotation_handle,
                egui::Align2::CENTER_CENTER,
                "\u{e41a}",
                egui::FontId::new(
                    19.0,
                    egui::FontFamily::Name("material_icons".into()),
                ),
                egui::Color32::WHITE,
            );
        }

        // 右下の外側に、比率を保った拡大・縮小ハンドルを表示する。
        if let Some(scale_handle) = scale_handle(&corners) {
            painter.line_segment(
                [corners[2], scale_handle],
                egui::Stroke::new(1.0, outline),
            );
            painter.circle_filled(scale_handle, SCALE_HANDLE_RADIUS, egui::Color32::WHITE);
            painter.circle_stroke(
                scale_handle,
                SCALE_HANDLE_RADIUS,
                egui::Stroke::new(1.5, outline),
            );
            painter.text(
                scale_handle,
                egui::Align2::CENTER_CENTER,
                "\u{e5d0}",
                egui::FontId::new(
                    16.0,
                    egui::FontFamily::Name("material_icons".into()),
                ),
                outline,
            );
        }

        // 各頂点にドラッグ可能な角ハンドルを表示する。
        for corner in corners {
            painter.circle_filled(corner, HANDLE_RADIUS, egui::Color32::WHITE);
            painter.circle_stroke(corner, HANDLE_RADIUS, egui::Stroke::new(2.0, outline));
        }
    }
}

/// ポインター位置にある最前面の表示画像とドラッグ方法を返す。
fn hit_test(
    layers: &[MapLayer],
    selected_layer_id: Option<&str>,
    pointer: egui::Pos2,
    map_xyz: MapXYZ,
    screen: Screen,
    viewport: egui::Rect,
) -> Option<(String, DragMode, [egui::Pos2; 4])> {
    // レイヤー一覧は先頭ほど前面なので、その順に判定する。
    for layer in layers {
        let MapLayer::Picture(picture) = layer else {
            continue;
        };
        if !picture.visible || picture.opacity <= 0.0 {
            continue;
        }
        let corners = PictureLayerRenderer::screen_corners(picture, map_xyz, screen, viewport)?;

        // 選択中の画像では回転ハンドルと角ハンドルを画像本体より優先する。
        if selected_layer_id == Some(picture.id.as_str()) {
            if rotation_handle(&corners)
                .is_some_and(|handle| handle.distance(pointer) <= ROTATION_HANDLE_HIT_RADIUS)
            {
                return Some((picture.id.clone(), DragMode::Rotate, corners));
            }
            if scale_handle(&corners)
                .is_some_and(|handle| handle.distance(pointer) <= SCALE_HANDLE_HIT_RADIUS)
            {
                return Some((picture.id.clone(), DragMode::Scale, corners));
            }
            if let Some(index) = corners
                .iter()
                .position(|corner| corner.distance(pointer) <= HANDLE_HIT_RADIUS)
            {
                return Some((picture.id.clone(), DragMode::Corner(index), corners));
            }
        }

        // 画像内部のドラッグは画像全体の移動として扱う。
        if point_in_quad(pointer, &corners) {
            return Some((picture.id.clone(), DragMode::Translate, corners));
        }
    }
    None
}

/// ドラッグ種別に応じて四隅を変形し、WGS84の矩形へ変換する。
fn dragged_rect(
    drag: &PictureLayerDrag,
    pointer: egui::Pos2,
    map_xyz: MapXYZ,
    screen: Screen,
    viewport: egui::Rect,
) -> Option<Rect> {
    let delta = pointer - drag.start_pointer;
    let mut corners = drag.initial_corners;

    // 操作対象に応じて画面座標上の四隅を変形する。
    match drag.mode {
        DragMode::Corner(index) => corners[index] += delta,
        DragMode::Translate => {
            for corner in &mut corners {
                *corner += delta;
            }
        }
        DragMode::Rotate => {
            let center = centroid(&corners);
            let start_angle = (drag.start_pointer.y - center.y)
                .atan2(drag.start_pointer.x - center.x);
            let current_angle = (pointer.y - center.y).atan2(pointer.x - center.x);
            corners = rotate_points(corners, center, current_angle - start_angle);
        }
        DragMode::Scale => {
            let center = centroid(&corners);
            let start_distance = drag.start_pointer.distance(center);
            if start_distance <= f32::EPSILON {
                return None;
            }
            let factor = (pointer.distance(center) / start_distance).clamp(0.05, 20.0);
            corners = scale_points(corners, center, factor);
        }
    }

    // 交差や退化が発生する形状は保存しない。
    if !is_convex_quad(&corners) {
        return None;
    }

    // 画面座標の四隅を地理座標へ戻し、経緯度の有効範囲に制限する。
    let geographic = corners.map(|point| {
        engine::pixel_xy_to_lon_lat(
            (
                f64::from(point.x - viewport.min.x),
                f64::from(point.y - viewport.min.y),
            ),
            &map_xyz,
            screen,
        )
        .ok()
        .map(|(longitude, latitude)| {
            (
                longitude.clamp(-180.0, 180.0),
                latitude.clamp(-90.0, 90.0),
            )
        })
    });
    let geographic = [
        geographic[0]?,
        geographic[1]?,
        geographic[2]?,
        geographic[3]?,
    ];

    // 画面上の頂点順をRectの名前付きフィールドへ対応付ける。
    Some(Rect {
        top_left: geographic[0],
        top_right: geographic[1],
        bottom_right: geographic[2],
        bottom_left: geographic[3],
    })
}

/// 上辺の外側に配置する回転ハンドルの位置を計算する。
fn rotation_handle(corners: &[egui::Pos2; 4]) -> Option<egui::Pos2> {
    let top_edge = corners[1] - corners[0];
    let length = top_edge.length();

    // 上辺が退化している場合は方向を決められない。
    if length <= f32::EPSILON {
        return None;
    }

    // 上辺の外向き法線に沿ってハンドルを配置する。
    let outward = egui::vec2(top_edge.y, -top_edge.x) / length;
    Some(midpoint(corners[0], corners[1]) + outward * ROTATION_HANDLE_OFFSET)
}

/// 右下の頂点から外側へ離した拡大・縮小ハンドルの位置を返す。
fn scale_handle(corners: &[egui::Pos2; 4]) -> Option<egui::Pos2> {
    let center = centroid(corners);
    let direction = corners[2] - center;
    let length = direction.length();
    if length <= f32::EPSILON {
        return None;
    }
    Some(corners[2] + direction / length * SCALE_HANDLE_OFFSET)
}

/// 2点を結ぶ線分の中点を返す。
fn midpoint(first: egui::Pos2, second: egui::Pos2) -> egui::Pos2 {
    egui::pos2((first.x + second.x) * 0.5, (first.y + second.y) * 0.5)
}

/// 四隅の平均位置を画像の回転中心として返す。
fn centroid(points: &[egui::Pos2; 4]) -> egui::Pos2 {
    egui::pos2(
        points.iter().map(|point| point.x).sum::<f32>() * 0.25,
        points.iter().map(|point| point.y).sum::<f32>() * 0.25,
    )
}

/// 指定した中心と角度を使って四隅を画面座標上で回転する。
fn rotate_points(
    points: [egui::Pos2; 4],
    center: egui::Pos2,
    angle: f32,
) -> [egui::Pos2; 4] {
    // 同じ正弦・余弦を4頂点に適用する。
    let (sin, cos) = angle.sin_cos();
    points.map(|point| {
        let x = point.x - center.x;
        let y = point.y - center.y;
        egui::pos2(
            center.x + x * cos - y * sin,
            center.y + x * sin + y * cos,
        )
    })
}

/// 中心を固定し、各頂点を同じ倍率で拡大・縮小する。
fn scale_points(
    points: [egui::Pos2; 4],
    center: egui::Pos2,
    factor: f32,
) -> [egui::Pos2; 4] {
    points.map(|point| center + (point - center) * factor)
}

/// 四隅が交差せず、面積を持つ凸四角形か判定する。
fn is_convex_quad(points: &[egui::Pos2; 4]) -> bool {
    let mut sign = 0.0_f32;

    // 隣り合う辺の外積の符号が全て一致することを確認する。
    for index in 0..4 {
        let first = points[(index + 1) % 4] - points[index];
        let second = points[(index + 2) % 4] - points[(index + 1) % 4];
        let cross = first.x * second.y - first.y * second.x;
        if cross.abs() <= 0.01 {
            return false;
        }
        if sign == 0.0 {
            sign = cross.signum();
        } else if cross.signum() != sign {
            return false;
        }
    }
    true
}

/// レイキャスティングで点が四角形内部にあるか判定する。
fn point_in_quad(point: egui::Pos2, corners: &[egui::Pos2; 4]) -> bool {
    let mut inside = false;
    let mut previous = 3;

    // 水平レイと辺の交差回数が奇数なら内部とみなす。
    for current in 0..4 {
        let first = corners[current];
        let second = corners[previous];
        if (first.y > point.y) != (second.y > point.y)
            && point.x
                < (second.x - first.x) * (point.y - first.y) / (second.y - first.y) + first.x
        {
            inside = !inside;
        }
        previous = current;
    }
    inside
}

#[cfg(test)]
mod tests {
    use super::{centroid, is_convex_quad, point_in_quad, rotate_points, scale_points};
    use eframe::egui;

    /// テスト用の画面座標上の矩形を返す。
    fn rectangle() -> [egui::Pos2; 4] {
        [
            egui::pos2(0.0, 0.0),
            egui::pos2(100.0, 0.0),
            egui::pos2(100.0, 60.0),
            egui::pos2(0.0, 60.0),
        ]
    }

    #[test]
    fn hit_test_distinguishes_inside_and_outside() {
        // 内部点と外部点をそれぞれ判定する。
        let corners = rectangle();
        assert!(point_in_quad(egui::pos2(50.0, 30.0), &corners));
        assert!(!point_in_quad(egui::pos2(120.0, 30.0), &corners));
    }

    #[test]
    fn rotation_preserves_center_and_convexity() {
        // 90度回転後も中心と凸形状が保たれることを確認する。
        let corners = rectangle();
        let center = centroid(&corners);
        let rotated = rotate_points(corners, center, std::f32::consts::FRAC_PI_2);

        assert!(is_convex_quad(&rotated));
        assert!((centroid(&rotated).distance(center)) < 0.001);
    }

    #[test]
    fn scaling_preserves_center_and_aspect_ratio() {
        // 中心を保ち、横幅と高さを同じ倍率で変更する。
        let corners = rectangle();
        let center = centroid(&corners);
        let scaled = scale_points(corners, center, 1.5);

        assert!((centroid(&scaled).distance(center)) < 0.001);
        assert!((scaled[0].distance(scaled[1]) - 150.0).abs() < 0.001);
        assert!((scaled[1].distance(scaled[2]) - 90.0).abs() < 0.001);
    }

    #[test]
    fn self_intersecting_quad_is_rejected() {
        // 辺が交差する四角形は編集結果として許可しない。
        let crossed = [
            egui::pos2(0.0, 0.0),
            egui::pos2(100.0, 60.0),
            egui::pos2(100.0, 0.0),
            egui::pos2(0.0, 60.0),
        ];
        assert!(!is_convex_quad(&crossed));
    }
}