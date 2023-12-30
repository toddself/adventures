use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::tilemap::TileDesc;

#[derive(Debug, Error)]
pub enum TileDataError {
    #[error("Index {0},{1} exceeds size")]
    OutOfBoundsError(u32, u32),

    #[error("Tile at {0},{1} not found")]
    NotFoundError(u32, u32),
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TileData {
    data: HashMap<u32, HashMap<u32, TileDesc>>,
    pub max_x: u32,
    pub max_y: u32,
}

impl TileData {
    pub fn new(max_x: u32, max_y: u32, tiles: Option<Vec<TileDesc>>) -> Result<Self> {
        let mut td = TileData {
            max_x,
            max_y,
            data: HashMap::new(),
        };

        if let Some(tiles) = tiles {
            let mut x = 0;
            let mut y = 0;
            for t in tiles.into_iter() {
                td.set_tile(x, y, t)?;
                if y + 1 == td.max_y {
                    y = 0;
                    x += 1;
                } else {
                    y += 1;
                }
            }
        }

        Ok(td)
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile: TileDesc) -> Result<()> {
        if x > self.max_x || y > self.max_y {
            return Err(TileDataError::OutOfBoundsError(x, y).into());
        }
        let mut col = HashMap::new();
        if let Some(col) = self.data.get_mut(&x) {
            col.insert(y, tile);
        } else {
            col.insert(y, tile);
        };
        self.data.insert(x, col);
        Ok(())
    }

    pub fn get_tile(&self, x: u32, y: u32) -> Result<&TileDesc> {
        if x > self.max_x || y > self.max_y {
            return Err(TileDataError::NotFoundError(x, y).into());
        }

        let col = self
            .data
            .get(&x)
            .ok_or(TileDataError::NotFoundError(x, y))?;
        col.get(&y).ok_or(TileDataError::NotFoundError(x, y).into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_from_vec() -> Result<()> {
        Ok(())
    }
}
