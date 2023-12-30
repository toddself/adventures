use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::tilemap::TileDesc;

#[derive(Debug, Error)]
pub enum TileDataError {
    #[error("Index {0}, {1} exceeds size")]
    OutOfBoundsError(u32, u32),

    #[error("Tile at {0}, {1} not found")]
    NotFoundError(u32, u32),
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TileData {
    data: HashMap<u32, HashMap<u32, TileDesc>>,
    pub max_x: u32,
    pub max_y: u32,
    curr_x: u32,
    curr_y: u32,
}

impl TileData {
    pub fn new(max_x: u32, max_y: u32) -> Self {
        TileData {
            max_x,
            max_y,
            curr_x: 0,
            curr_y: 0,
            data: HashMap::new(),
        }
    }

    pub fn from_vec(max_x: u32, max_y: u32, tiles: Vec<TileDesc>) -> Result<Self> {
        let mut td = TileData {
            max_x,
            max_y,
            curr_x: 0,
            curr_y: 0,
            data: HashMap::new(),
        };

        for t in tiles.into_iter() {
            td.set_tile(t)?;
        }

        println!("{:?}", td.data);

        Ok(td)
    }

    pub fn set_tile(&mut self, tile: TileDesc) -> Result<()> {
        let (x, y) = tile.coords.into();
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

    pub fn get_tile(&self, x: &u32, y: &u32) -> Result<&TileDesc> {
        if *x > self.max_x || *y > self.max_y {
            return Err(TileDataError::NotFoundError(*x, *y).into());
        }

        let col = self
            .data
            .get(x)
            .ok_or(TileDataError::NotFoundError(*x, *y))?;
        col.get(y)
            .ok_or(TileDataError::NotFoundError(*x, *y).into())
    }
}

impl Iterator for TileData {
    type Item = TileDesc;

    fn next(&mut self) -> Option<Self::Item> {
        match self.data.get(&self.curr_x) {
            Some(col) => {
                self.curr_x = if self.curr_x + 1 > self.max_x {
                    0
                } else {
                    self.curr_x + 1
                };

                let val = col.get(&self.curr_y).copied();

                self.curr_y = if self.curr_y + 1 > self.max_y {
                    0
                } else {
                    self.curr_y + 1
                };

                val
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tile_coords::TileCoords;

    use super::*;

    #[test]
    fn set_and_get() -> Result<()> {
        let mut td = TileData::new(2, 2);
        td.set_tile(TileDesc::new(0, TileCoords::new(0, 0), None))?;
        Ok(())
    }

    #[test]
    fn new_from_vec() -> Result<()> {
        let v = vec![
            TileDesc::new(0, TileCoords::new(0, 0), None),
            TileDesc::new(0, TileCoords::new(1, 0), None),
        ];

        let td = TileData::from_vec(2, 2, v)?;

        let t1 = td.get_tile(&0, &0)?;
        let t2 = td.get_tile(&0, &0)?;
        assert_eq!(t1, &TileDesc::new(0, TileCoords::new(0, 0), None));
        assert_eq!(t2, &TileDesc::new(0, TileCoords::new(1, 0), None));

        Ok(())
    }
}
