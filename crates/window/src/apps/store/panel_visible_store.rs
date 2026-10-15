pub struct PanelVisibleStore {
    pub side_panel: bool,
    add_raster_tile_dialog: bool,
}

pub trait PanelVisibleStoreTrait {
    fn toggle_side_panel(&mut self);
    fn open_add_raster_tile_dialog(&mut self);
    fn close_add_raster_tile_dialog(&mut self);
    fn is_add_raster_tile_dialog_open(&self) -> bool;
}

impl PanelVisibleStoreTrait for PanelVisibleStore {
    /// サイドパネルの表示/非表示を切り替える。
    ///
    /// # Arguments
    /// * `side_panel` - サイドパネルの表示/非表示
    fn toggle_side_panel(&mut self) {
        self.side_panel = !self.side_panel;
    }

    fn open_add_raster_tile_dialog(&mut self) {
        self.add_raster_tile_dialog = true;
    }

    fn close_add_raster_tile_dialog(&mut self) {
        self.add_raster_tile_dialog = false;
    }

    fn is_add_raster_tile_dialog_open(&self) -> bool {
        self.add_raster_tile_dialog
    }
}

impl Default for PanelVisibleStore {
    fn default() -> Self {
        Self {
            side_panel: true,
            add_raster_tile_dialog: false,
        }
    }
}
