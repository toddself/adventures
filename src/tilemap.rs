use std::fs;

use anyhow::{anyhow, Result};
use bevy::{math::ivec3, prelude::*};
use bevy_simple_tilemap::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MapScreen {
    pub tile_map: String,
    pub tile_rows: u32,
    pub tile_cols: u32,
    pub data: Vec<TileDesc>,
}

impl MapScreen {
    pub fn new_from_file(filename: &str) -> Result<MapScreen> {
        let file_data = fs::read_to_string(filename)?;
        match ron::from_str(&file_data) {
            Ok(ms) => Ok(ms),
            Err(e) => Err(anyhow!(e)),
        }
    }

    pub fn tilemap_from_struct(&self) -> Vec<(IVec3, Option<Tile>)> {
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileDesc {
    tile_source: (u32, u32),
    screen_pos: (i32, i32, i32),
}
