use std::fs;

use anyhow::{anyhow, Result};
use bevy::{
    math::{ivec3, vec2},
    prelude::*,
};
use bevy_simple_tilemap::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::Wall;
use crate::settings::GameSettings;

// converts a coordinate (origin bottom left) to an screen position
pub fn coord_to_screen_pos(x: i32, y: i32, z: f32, settings: &GameSettings) -> Vec3 {
    let x_trans = match settings.is_editor {
        true => settings.editor_area_x_transform,
        false => settings.game_area_x_transform,
    };
    let new_x = (x as f32 * settings.tile_width * settings.scale) + x_trans;
    let new_y = (y as f32 * settings.tile_height * settings.scale) + settings.game_area_y_transform;
    Vec3::new(new_x, new_y, z)
}

pub fn screen_pos_to_coord(coords: Vec3, settings: &GameSettings) -> (i32, i32) {
    let x_trans = match settings.is_editor {
        true => settings.editor_area_x_transform,
        false => settings.game_area_x_transform,
    };

    let tile_x = settings.tile_width * settings.scale;
    let tile_y = settings.tile_height * settings.scale;
    let new_x = ((x_trans - coords.x) * -1.) / tile_x;
    let new_y = ((settings.game_area_y_transform - coords.y) * -1.) / tile_y;
    (new_x.floor() as i32, new_y.floor() as i32)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapScreen {
    pub tile_map: String,
    pub tile_rows: u32,
    pub tile_cols: u32,
    pub data: Vec<TileDesc>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TileType {
    Wall,
}

#[derive(Component)]
struct MapTile;

impl MapScreen {
    pub fn new(filename: &str, rows: u32, cols: u32) -> Self {
        MapScreen {
            tile_map: filename.to_owned(),
            tile_rows: rows,
            tile_cols: cols,
            data: vec![],
        }
    }

    pub fn new_from_file(filename: &str) -> Result<Self> {
        let file_data = fs::read_to_string(filename)?;
        match ron::from_str(&file_data) {
            Ok(ms) => Ok(ms),
            Err(e) => Err(anyhow!("{}, {:?}", filename, e)),
        }
    }

    pub fn tilemapdata_from_struct(&self) -> Vec<(IVec3, Option<Tile>)> {
        self.data
            .iter()
            .map(|t| {
                let sprite_index = t.tile_source.1 + (t.tile_source.0 * self.tile_rows);
                let tile = Tile {
                    sprite_index,
                    ..default()
                };
                let v3 = ivec3(t.screen_pos.0, t.screen_pos.1, t.screen_pos.2);
                (v3, Some(tile))
            })
            .collect()
    }

    pub fn get_wallmap(&self, settings: &GameSettings) -> Vec<(SpatialBundle, Wall)> {
        self.data
            .iter()
            .filter(|t| t.metadata == Some(TileType::Wall))
            .map(|t| {
                let pos = coord_to_screen_pos(t.screen_pos.0, t.screen_pos.1, 1.0, settings);
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
        let texture_handle = asset_server.load(&self.tile_map);
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
        tilemap.set_tiles(self.tilemapdata_from_struct());

        let x_trans = match settings.is_editor {
            true => settings.editor_area_x_transform,
            false => settings.game_area_x_transform,
        };

        TileMapBundle {
            tilemap,
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(x_trans, settings.game_area_y_transform, 0.0),
                scale: Vec3::splat(settings.scale),
                ..default()
            },
            ..default()
        }
    }
}

impl Default for MapScreen {
    fn default() -> Self {
        MapScreen {
            tile_map: String::new(),
            tile_rows: 16,
            tile_cols: 16,
            data: vec![],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileDesc {
    tile_source: (u32, u32),
    screen_pos: (i32, i32, i32),
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
        };
        let gs = GameSettings::new_from_sf(&sf, false);
        let pos0 = coord_to_screen_pos(0, 0, 0.0, &gs);
        assert_eq!(pos0.x, gs.game_area_x_transform);
        assert_eq!(pos0.y, gs.game_area_y_transform);
        println!("pos0 {:?}", pos0);
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
        };
        let gs = GameSettings::new_from_sf(&sf, false);
        let pos0 = screen_pos_to_coord(
            Vec3::new(gs.game_area_x_transform, gs.game_area_y_transform, 0.0),
            &gs,
        );
        assert_eq!(pos0, (0, 0));
        let pos1 = screen_pos_to_coord(
            Vec3::new(
                gs.game_area_x_transform + (gs.scale * gs.tile_width),
                gs.game_area_y_transform + (gs.scale * gs.tile_height),
                0.0,
            ),
            &gs,
        );
        assert_eq!(pos1, (1, 1));

        // anywhere in tile selects proper tile
        let pos2 = screen_pos_to_coord(
            Vec3::new(
                gs.game_area_x_transform + (gs.scale * gs.tile_width * 4. + 2.),
                gs.game_area_y_transform + (gs.scale * gs.tile_height * 4. + 2.),
                0.0,
            ),
            &gs,
        );
        assert_eq!(pos2, (4, 4));
        Ok(())
    }
}
