pub struct PanelVisibleStore {
    pub side_panel: bool,
}

pub trait PanelVisibleStoreTrait {
    fn toggle_side_panel(&mut self);
}

impl PanelVisibleStoreTrait for PanelVisibleStore {
    /// サイドパネルの表示/非表示を切り替える。
    ///
    /// # Arguments
    /// * `side_panel` - サイドパネルの表示/非表示
    fn toggle_side_panel(&mut self) {
        self.side_panel = !self.side_panel;
    }
}

impl Default for PanelVisibleStore {
    fn default() -> Self {
        Self { side_panel: true }
    }
}
