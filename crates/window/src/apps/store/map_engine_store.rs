use domain::params::map_config::{MAP_CENTER_LATITUDE, MAP_CENTER_LONGITUDE, MAP_ZOOM_LEVEL};
use domain::types::gis_engine_type::{MapEngineInput, MapObject, MapXYZ, MoveXY, Screen};

pub struct MapEngineStore {
    input: MapEngineInput,
    map_object: Option<MapObject>,
}

pub trait MapEngineStoreTrait {
    fn set_xyz(&mut self, map_xyz: MapXYZ);
    fn get_xyz(&self) -> &MapXYZ;

    fn set_zoom(&mut self, zoom: u32);
    fn get_zoom(&self) -> u32;

    fn set_screen(&mut self, screen: Screen);
    fn get_screen(&self) -> &Screen;

    fn get_map_engine_input(&self) -> MapEngineInput;

    fn set_map_object(&mut self, map_object: MapObject);
    fn get_map_object(&self) -> Option<&MapObject>;
}

impl MapEngineStoreTrait for MapEngineStore {
    fn set_xyz(&mut self, map_xyz: MapXYZ) {
        self.input.map_xyz = map_xyz;
    }

    fn get_xyz(&self) -> &MapXYZ {
        &self.input.map_xyz
    }

    fn set_zoom(&mut self, zoom: u32) {
        self.input.map_xyz.zoom = zoom;
    }

    fn get_zoom(&self) -> u32 {
        self.input.map_xyz.zoom
    }

    fn set_screen(&mut self, screen: Screen) {
        self.input.screen = screen;
    }

    fn get_screen(&self) -> &Screen {
        &self.input.screen
    }

    fn get_map_engine_input(&self) -> MapEngineInput {
        self.input
    }
    fn set_map_object(&mut self, map_object: MapObject) {
        self.map_object = Some(map_object);
    }
    fn get_map_object(&self) -> Option<&MapObject> {
        self.map_object.as_ref()
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
            map_object: None,
        }
    }
}
