use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use bevy::{
    asset::{AssetServer, Assets},
    ecs::system::{Res, ResMut},
    math::{ivec3, vec2, IVec3, Vec3},
    render::prelude::SpatialBundle,
    sprite::TextureAtlas,
    transform::components::Transform,
};
use bevy_simple_tilemap::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{components::Wall, tile_data::TileData};
use crate::settings::GameSettings;
use crate::tile_coords::{coord_to_screen_pos, TileCoords};

#[derive(Debug, Serialize, Deserialize)]
pub struct MapScreen {
    pub map_name: String,
    pub map_id: uuid::Uuid,
    pub tile_set: Option<PathBuf>,
    pub tile_rows: u32,
    pub tile_cols: u32,
    pub tile_data: TileData,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Copy, Clone)]
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
            tile_set: None,
            tile_data: TileData::default()
        }
    }
}

impl MapScreen {
    pub fn new(rows: u32, cols: u32, map_name: Option<&str>, filename: Option<&str>) -> Self {
        MapScreen {
            map_name: map_name.unwrap_or_default().to_owned(),
            map_id: uuid::Uuid::new_v4(),
            tile_set: Some(filename.unwrap_or_default().to_owned().into()),
            tile_rows: rows,
            tile_cols: cols,
            tile_data: TileData::default(),
        }
    }

    pub fn new_from_file(filename: &str) -> Result<Self> {
        let file_data = fs::read_to_string(filename)?;
        match ron::from_str(&file_data) {
            Ok(ms) => Ok(ms),
            Err(e) => Err(anyhow!("{}, {:?}", filename, e)),
        }
    }

    pub fn tilemapdata_from_struct(&self, tile_z: f32) -> Option<Vec<(IVec3, Option<Tile>)>> {
        let mut data = vec![];
        for t in self.tile_data.iter() {
            // let sprite_index = t.tile_source.1 + (t.tile_source.0 * self.tile_rows);
            let tile = Tile {
                sprite_index: t.tile_index,
                ..bevy::utils::default()
            };
            let xi32 = i32::try_from(t.coords.x()).ok()?;
            let yi32 = i32::try_from(t.coords.y()).ok()?;
            let v3 = ivec3(xi32, yi32, tile_z.floor() as i32);
            data.push((v3, Some(tile)))
        }
        Some(data)
    }

    pub fn get_wallmap(&self, settings: &GameSettings) -> Vec<(SpatialBundle, Wall)> {
        self.tile_data
            .iter()
            .filter(|t| t.metadata == Some(TileType::Wall))
            .map(|t| {
                let pos =
                    coord_to_screen_pos(t.coords.x(), t.coords.y(), settings.game_z, settings);
                (
                    SpatialBundle {
                        transform: Transform {
                            translation: pos,
                            scale: Vec3::splat(settings.scale),
                            ..bevy::utils::default()
                        },
                        ..bevy::utils::default()
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
    ) -> Option<TileMapBundle> {
        let tm = match &self.tile_set {
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
        match self.tilemapdata_from_struct(settings.tile_z) {
            Some(data) => {
                tilemap.set_tiles(data);

                Some(TileMapBundle {
                    tilemap,
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            settings.game_area_x_transform,
                            settings.game_area_y_transform,
                            0.0,
                        ),
                        scale: Vec3::splat(settings.scale),
                        ..bevy::utils::default()
                    },
                    ..bevy::utils::default()
                })
            }
            None => None,
        }
    }
}

#[derive(Debug, Serialize, Copy, Deserialize, Clone, PartialEq)]
pub struct TileDesc {
    tile_index: u32,
    pub coords: TileCoords,
    metadata: Option<TileType>,
}

impl TileDesc {
    pub fn new(index: u32, coords: TileCoords, metadata: Option<TileType>) -> TileDesc {
        TileDesc {
            tile_index: index,
            coords,
            metadata,
        }
    }
}
