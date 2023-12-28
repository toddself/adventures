use std::{fmt::Display, fs, path::PathBuf};

use anyhow::{anyhow, Result};
use bevy::{
    math::{ivec3, vec2},
    prelude::*,
};
use bevy_simple_tilemap::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::Wall;
use crate::settings::GameSettings;

#[derive(Debug, PartialEq)]
pub struct TileCoords(i32, i32);

impl Display for TileCoords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

// converts a coordinate (origin bottom left) to an screen position
pub fn coord_to_screen_pos(x: i32, y: i32, z: f32, settings: &GameSettings) -> Vec3 {
    let new_x = (x as f32 * settings.tile_width * settings.scale) + settings.game_area_x_transform;
    let new_y = (y as f32 * settings.tile_height * settings.scale) + settings.game_area_y_transform;
    Vec3::new(new_x, new_y, z)
}

pub fn screen_pos_to_coord(coords: Vec3, settings: &GameSettings) -> TileCoords {
    let tile_x = settings.tile_width * settings.scale;
    let tile_y = settings.tile_height * settings.scale;

    let new_x = ((settings.game_area_x_transform - coords.x) * -1.) / tile_x;
    let new_y = ((settings.game_area_y_transform - coords.y) * -1.) / tile_y;

    TileCoords(new_x.floor() as i32, new_y.floor() as i32)
}

pub fn top_left_to_coord(coords: Vec3, settings: &GameSettings) -> TileCoords {
    let y = (coords.y - settings.viewport_height).abs();
    let x = coords.x - settings.left_margin;

    let tile_x = settings.tile_width * settings.scale;
    let tile_y = settings.tile_height * settings.scale;

    let new_x = x / tile_x;
    let new_y = y / tile_y;

    TileCoords(new_x as i32, new_y as i32)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapScreen {
    pub map_name: String,
    pub map_id: uuid::Uuid,
    pub tile_map: Option<PathBuf>,
    pub tile_rows: u32,
    pub tile_cols: u32,
    pub tile_data: Vec<TileDesc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Door,
    Item,
    Enemy,
    NPC,
}

impl Default for MapScreen {
    fn default() -> Self {
        MapScreen {
            tile_rows: 24,
            tile_cols: 18,
            map_name: String::default(),
            map_id: uuid::Uuid::default(),
            tile_map: None,
            tile_data: vec![],
        }
    }
}

impl MapScreen {
    pub fn new(rows: u32, cols: u32, map_name: Option<&str>, filename: Option<&str>) -> Self {
        MapScreen {
            map_name: map_name.unwrap_or_default().to_owned(),
            map_id: uuid::Uuid::new_v4(),
            tile_map: Some(filename.unwrap_or_default().to_owned().into()),
            tile_rows: rows,
            tile_cols: cols,
            tile_data: vec![],
        }
    }

    pub fn new_from_file(filename: &str) -> Result<Self> {
        let file_data = fs::read_to_string(filename)?;
        match ron::from_str(&file_data) {
            Ok(ms) => Ok(ms),
            Err(e) => Err(anyhow!("{}, {:?}", filename, e)),
        }
    }

    pub fn tilemapdata_from_struct(&self, tile_z: f32) -> Vec<(IVec3, Option<Tile>)> {
        self.tile_data
            .iter()
            .map(|t| {
                // let sprite_index = t.tile_source.1 + (t.tile_source.0 * self.tile_rows);
                let tile = Tile {
                    sprite_index: t.tile_index,
                    ..default()
                };
                let v3 = ivec3(t.x, t.y, tile_z.floor() as i32);
                (v3, Some(tile))
            })
            .collect()
    }

    pub fn get_wallmap(&self, settings: &GameSettings) -> Vec<(SpatialBundle, Wall)> {
        self.tile_data
            .iter()
            .filter(|t| t.metadata == Some(TileType::Wall))
            .map(|t| {
                let pos = coord_to_screen_pos(t.x, t.y, settings.game_z, settings);
                (
                    SpatialBundle {
                        transform: Transform {
                            translation: pos,
                            scale: Vec3::splat(settings.scale),
                            ..default()
                        },
                        ..default()
                    },
                    Wall,
                )
            })
            .collect()
    }

    pub fn get_tilemap(
        &self,
        settings: &GameSettings,
        asset_server: &Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> TileMapBundle {
        let tm = match &self.tile_map {
            Some(tm) => tm.clone(),
            None => {
                println!("Tile map did not exist!");
                std::process::exit(1)
            }
        };
        let texture_handle = asset_server.load(tm);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            vec2(settings.tile_width, settings.tile_height),
            self.tile_cols as usize,
            self.tile_rows as usize,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut tilemap = TileMap::default();
        tilemap.set_tiles(self.tilemapdata_from_struct(settings.tile_z));

        TileMapBundle {
            tilemap,
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(
                    settings.game_area_x_transform,
                    settings.game_area_y_transform,
                    0.0,
                ),
                scale: Vec3::splat(settings.scale),
                ..default()
            },
            ..default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileDesc {
    tile_index: u32,
    x: i32,
    y: i32,
    metadata: Option<TileType>,
}

#[cfg(test)]
mod tests {
    use crate::settings::SettingsFile;

    use super::*;

    #[test]
    fn coord_to_screen_pos_test() -> Result<()> {
        let sf = SettingsFile {
            scale: 1.,
            x_max: 24.,
            y_max: 18.,
            input_debounce: 0.04,
            tile_height: 16.,
            tile_width: 16.,
            tile_z: 0.0,
            game_z: 1.0,
        };
        let gs = GameSettings::new_from_sf(&sf, false);
        let pos0 = coord_to_screen_pos(0, 0, 0.0, &gs);
        assert_eq!(pos0.x, gs.game_area_x_transform);
        assert_eq!(pos0.y, gs.game_area_y_transform);
        let pos1 = coord_to_screen_pos(1, 1, 0.0, &gs);
        assert_eq!(
            pos1.x,
            gs.game_area_x_transform + (gs.scale * gs.tile_width)
        );
        assert_eq!(
            pos1.y,
            gs.game_area_y_transform + (gs.scale * gs.tile_width)
        );
        Ok(())
    }

    #[test]
    fn screen_pos_to_coord_test() -> Result<()> {
        let sf = SettingsFile {
            scale: 1.,
            x_max: 24.,
            y_max: 18.,
            input_debounce: 0.04,
            tile_height: 16.,
            tile_width: 16.,
            tile_z: 0.0,
            game_z: 1.0,
        };
        let gs = GameSettings::new_from_sf(&sf, false);
        let pos0 = screen_pos_to_coord(
            Vec3::new(gs.game_area_x_transform, gs.game_area_y_transform, 0.0),
            &gs,
        );
        assert_eq!(pos0, TileCoords(0, 0));
        let pos1 = screen_pos_to_coord(
            Vec3::new(
                gs.game_area_x_transform + (gs.scale * gs.tile_width),
                gs.game_area_y_transform + (gs.scale * gs.tile_height),
                0.0,
            ),
            &gs,
        );
        assert_eq!(pos1, TileCoords(1, 1));

        // anywhere in tile selects proper tile
        let pos2 = screen_pos_to_coord(
            Vec3::new(
                gs.game_area_x_transform + (gs.scale * gs.tile_width * 4. + 2.),
                gs.game_area_y_transform + (gs.scale * gs.tile_height * 4. + 2.),
                0.0,
            ),
            &gs,
        );
        assert_eq!(pos2, TileCoords(4, 4));
        Ok(())
    }

    #[test]
    fn screen_pos_to_coord_editor_test() -> Result<()> {
        let sf = SettingsFile {
            scale: 1.,
            x_max: 24.,
            y_max: 18.,
            input_debounce: 0.04,
            tile_height: 16.,
            tile_width: 16.,
            tile_z: 0.0,
            game_z: 1.0,
        };
        let gs = GameSettings::new_from_sf(&sf, true);
        let screen_pos = Vec3::new(gs.game_area_x_transform, gs.game_area_y_transform, 0.0);
        let pos0 = screen_pos_to_coord(screen_pos, &gs);
        assert_eq!(pos0, TileCoords(0, 0));
        let pos1 = screen_pos_to_coord(
            Vec3::new(
                gs.game_area_x_transform + (gs.scale * gs.tile_width),
                gs.game_area_y_transform + (gs.scale * gs.tile_height),
                0.0,
            ),
            &gs,
        );
        assert_eq!(pos1, TileCoords(1, 1));

        // anywhere in tile selects proper tile
        let pos2 = screen_pos_to_coord(
            Vec3::new(
                gs.game_area_x_transform + (gs.scale * gs.tile_width * 4. + 2.),
                gs.game_area_y_transform + (gs.scale * gs.tile_height * 4. + 2.),
                0.0,
            ),
            &gs,
        );
        assert_eq!(pos2, TileCoords(4, 4));
        Ok(())
    }

    #[test]
    fn top_left_to_coord_editor_test() -> Result<()> {
        let sf = SettingsFile {
            scale: 1.,
            x_max: 24.,
            y_max: 18.,
            input_debounce: 0.04,
            tile_height: 16.,
            tile_width: 16.,
            tile_z: 0.0,
            game_z: 1.0,
        };
        let gs = GameSettings::new_from_sf(&sf, true);
        let screen_pos = Vec3::new(gs.left_margin, gs.viewport_height, 0.);
        let pos0 = top_left_to_coord(screen_pos, &gs);
        assert_eq!(pos0, TileCoords(0, 0));

        let screen_pos = Vec3::new(gs.left_margin + 100., gs.viewport_height - 100., 0.);
        let pos0 = top_left_to_coord(screen_pos, &gs);
        assert_eq!(pos0, TileCoords(6, 6));
        Ok(())
    }
}
