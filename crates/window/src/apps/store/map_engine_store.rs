use domain::params::map_config::{MAP_CENTER_LATITUDE, MAP_CENTER_LONGITUDE, MAP_ZOOM_LEVEL};
use domain::types::gis_engine_type::{MapEngineInput, MapXYZ, MoveXY, Screen};

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
                    latitude: MAP_CENTER_LATITUDE,
                    longitude: MAP_CENTER_LONGITUDE,
                    zoom: MAP_ZOOM_LEVEL,
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
