use std::fs;

use anyhow::Result;
use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct SettingsFile {
    pub scale: f32,
    pub x_max: f32,
    pub y_max: f32,
    pub input_debounce: f32,
    pub tile_width: f32,
    pub tile_height: f32,
    pub tile_z: f32,
    pub game_z: f32
}

impl SettingsFile {
    pub fn new_from_file(filename: &str) -> Result<Self> {
        let data = fs::read_to_string(filename)?;
        let s = ron::from_str(&data)?;
        Ok(s)
    }
}

#[derive(Resource)]
pub struct GameSettings {
    pub scale: f32,
    pub game_area_tile_x_max: f32,
    pub game_area_tile_y_max: f32,
    pub input_debounce: f32,
    pub tile_width: f32,
    pub tile_height: f32,
    pub game_area_x_res: f32,
    pub game_area_y_res: f32,
    pub top_margin: f32,
    pub left_margin: f32,
    pub viewport_height: f32,
    pub editor_viewport_width: f32,
    pub game_area_x_transform: f32,
    pub game_area_y_transform: f32,
    pub editor_area_x_transform: f32,
    pub game_area_x_max: f32,
    pub game_area_x_min: f32,
    pub game_area_y_max: f32,
    pub game_area_y_min: f32,
    pub tile_z: f32,
    pub game_z: f32,
    pub is_editor: bool,
}

impl GameSettings {
    pub fn new_from_sf(sf: &SettingsFile, editor: bool) -> Self {
        Self::new(
            sf.scale,
            sf.x_max,
            sf.y_max,
            sf.input_debounce,
            sf.tile_width,
            sf.tile_height,
            sf.tile_z,
            sf.game_z,
            editor,
        )
    }

    pub fn new(
        scale: f32,
        game_area_tile_x_max: f32,
        game_area_tile_y_max: f32,
        input_debounce: f32,
        tile_width: f32,
        tile_height: f32,
        tile_z: f32,
        game_z: f32,
        editor: bool,
    ) -> Self {
        // figure out pixel dimensions of playable area
        let game_area_x_res: f32 = game_area_tile_x_max * (tile_width * scale);
        let game_area_y_res: f32 = game_area_tile_y_max * (tile_height * scale);

        // how big is the information section on top
        let top_margin: f32 = game_area_y_res / 5.0;

        // how big is the game editor right panel
        // this is not used in the game
        let left_margin: f32 = game_area_x_res / 2.0;

        // figure out how big the application is
        let viewport_height: f32 = top_margin + game_area_y_res;
        let editor_viewport_width: f32 = left_margin + game_area_x_res;

        // what is the x,y translation for the tile map to position them in the playable area
        // we use -1.0 since we want the origin (0,0) to be the lower left, and then we
        // need to move the vertical position lower to account for the top margin
        let game_area_x_transform: f32 =
            (game_area_x_res / 2.0) * -1.0 + (tile_width * scale / 2.0);
        let game_area_y_transform: f32 =
            (game_area_y_res / 2.0) * -1.0 + (tile_height * scale / 2.0) - top_margin / 2.0;

        let editor_area_x_transform: f32 =
            (game_area_x_res / 2.0) * -1.0 + (tile_height * scale / 2.0) + left_margin / 2.0;

        // figure out the pixel positions of the "walls" around the playable area
        let game_area_x_max: f32 = (game_area_x_res + game_area_x_transform) - (tile_width * scale);
        let game_area_x_min: f32 = 0.0 + game_area_x_transform;
        let game_area_y_max: f32 =
            (game_area_y_res + game_area_y_transform) - (tile_height * scale);
        let game_area_y_min: f32 = 0.0 + game_area_y_transform;

        GameSettings {
            scale,
            game_area_tile_x_max,
            game_area_tile_y_max,
            input_debounce,
            tile_width,
            tile_height,
            game_area_x_res,
            game_area_y_res,
            top_margin,
            left_margin,
            viewport_height,
            editor_viewport_width,
            game_area_x_transform,
            game_area_y_transform,
            editor_area_x_transform,
            game_area_x_max,
            game_area_x_min,
            game_area_y_max,
            game_area_y_min,
            tile_z,
            game_z,
            is_editor: editor,
        }
    }
}
