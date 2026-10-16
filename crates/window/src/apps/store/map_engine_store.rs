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
    
    fn get_map_engine_input(&self) -> MapEngineInput;

    fn set_map_object(&mut self, map_object: MapObject);
    fn get_map_object(&self) -> Option<&MapObject>;
}

impl MapEngineStoreTrait for MapEngineStore {
    /// 緯度・経度・ズームレベルを設定する。
    ///
    /// # Arguments
    /// * `map_xyz` - 緯度・経度・ズームレベル
    fn set_xyz(&mut self, map_xyz: MapXYZ) {
        self.input.map_xyz = map_xyz;
        // 地図オブジェクトをリセットする。
        self.map_object = None;
    }

    /// 緯度・経度・ズームレベルを取得する。
    ///
    /// # Returns
    /// 緯度・経度・ズームレベル
    fn get_xyz(&self) -> &MapXYZ {
        &self.input.map_xyz
    }

    /// ズームレベルを設定する。
    ///
    /// # Arguments
    /// * `zoom` - ズームレベル
    fn set_zoom(&mut self, zoom: u32) {
        self.input.map_xyz.zoom = zoom;
    }

    /// ズームレベルを取得する。
    ///
    /// # Returns
    /// ズームレベル
    fn get_zoom(&self) -> u32 {
        self.input.map_xyz.zoom
    }

    /// 画面サイズを設定する。
    ///     
    /// # Arguments
    /// * `screen` - 画面サイズ
    fn set_screen(&mut self, screen: Screen) {
        self.input.screen = screen;
    }

    /// 地図エンジンの入力を取得する。
    ///
    /// # Returns
    /// 地図エンジンの入力
    fn get_map_engine_input(&self) -> MapEngineInput {
        self.input
    }
    /// 地図オブジェクトを設定する。
    ///
    /// # Arguments
    /// * `map_object` - 地図オブジェクト
    fn set_map_object(&mut self, map_object: MapObject) {
        self.map_object = Some(map_object);
    }
    /// 地図オブジェクトを取得する。
    ///
    /// # Returns
    /// 地図オブジェクト
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
