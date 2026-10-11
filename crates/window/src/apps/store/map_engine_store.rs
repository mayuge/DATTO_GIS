use domain::{MapEngineInput, MapXYZ, MoveXY, Screen};

pub struct MapEngineStore {
    input: MapEngineInput,
}

pub trait MapEngineStoreTrait {
    fn set_xyz(&mut self, map_xyz: MapXYZ);
    fn get_xyz(&self) -> &MapXYZ;

    fn set_screen(&mut self, screen: Screen);
    fn get_screen(&self) -> &Screen;

    fn set_movement(&mut self, movement: MoveXY);
    fn get_movement(&self) -> &MoveXY;

    fn get_all(&self) -> &MapEngineInput;
}

impl MapEngineStoreTrait for MapEngineStore {
    fn set_xyz(&mut self, map_xyz: MapXYZ) {
        self.input.map_xyz = map_xyz;
    }

    fn get_xyz(&self) -> &MapXYZ {
        &self.input.map_xyz
    }

    fn set_screen(&mut self, screen: Screen) {
        self.input.screen = screen;
    }

    fn get_screen(&self) -> &Screen {
        &self.input.screen
    }

    fn set_movement(&mut self, movement: MoveXY) {
        self.input.movement = movement;
    }

    fn get_movement(&self) -> &MoveXY {
        &self.input.movement
    }
    fn get_all(&self) -> &MapEngineInput {
        &self.input
    }
}

impl Default for MapEngineStore {
    fn default() -> Self {
        Self {
            input: MapEngineInput {
                map_xyz: MapXYZ {
                    latitude: 35.6812,
                    longitude: 139.7671,
                    zoom: 6,
                },
                screen: Screen {
                    width: 1.0,
                    height: 1.0,
                },
                movement: MoveXY {
                    move_x: 0.0,
                    move_y: 0.0,
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MapEngineStore, MapEngineStoreTrait};

    #[test]
    fn init_sets_default_map_view_and_allows_state_updates() {
        let mut store = MapEngineStore::default();
        store.init();

        assert_eq!(store.get_xyz().latitude, 35.6812);
        assert_eq!(store.get_xyz().longitude, 139.7671);
        assert_eq!(store.get_xyz().zoom, 6);

        store.set_xyz(domain::MapXYZ {
            latitude: 40.0,
            longitude: 141.0,
            zoom: 8,
        });
        assert_eq!(store.get_xyz().latitude, 40.0);
        assert_eq!(store.get_xyz().longitude, 141.0);
        assert_eq!(store.get_xyz().zoom, 8);
    }

    #[test]
    fn engine_creates_tiles_for_the_stored_view() {
        let store = MapEngineStore::default();
        let map = engine::create_map_obj(
            *store.get_xyz(),
            domain::Screen {
                width: 1280.0,
                height: 800.0,
            },
            *store.get_movement(),
        )
        .expect("既定の表示範囲からタイルを計算できる必要があります");

        assert!(!map.tiles.is_empty());
    }
}
