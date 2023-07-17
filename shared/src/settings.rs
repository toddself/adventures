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
    pub viewport_height: f32,
    pub game_area_x_transform: f32,
    pub game_area_y_transform: f32,
    pub game_area_x_max: f32,
    pub game_area_x_min: f32,
    pub game_area_y_max: f32,
    pub game_area_y_min: f32,
}

impl GameSettings {
    pub fn new(
        scale: f32,
        game_area_tile_x_max: f32,
        game_area_tile_y_max: f32,
        input_debounce: f32,
        tile_width: f32,
        tile_height: f32,
    ) -> Self {
        let game_area_x_res: f32 = game_area_tile_x_max * (tile_width * scale);
        let game_area_y_res: f32 = game_area_tile_y_max * (tile_height * scale);
        let top_margin: f32 = game_area_y_res / 5.0;
        let viewport_height: f32 = top_margin + game_area_y_res;
        let game_area_x_transform: f32 =
            (game_area_x_res / 2.0) * -1.0 + (tile_width * scale / 2.0);
        let game_area_y_transform: f32 =
            (game_area_y_res / 2.0) * -1.0 + (tile_height * scale / 2.0) - top_margin / 2.0;

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
            viewport_height,
            game_area_x_transform,
            game_area_y_transform,
            game_area_x_max,
            game_area_x_min,
            game_area_y_max,
            game_area_y_min,
        }
    }
}
