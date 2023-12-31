use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::tilemap::TileDesc;

#[derive(Debug, Error)]
pub enum TileDataError {
    #[error("Index {0}, {1} exceeds size {2}, {3}")]
    OutOfBoundsError(u32, u32, u32, u32),

    #[error("Tile at {0}, {1} not found")]
    NotFoundError(u32, u32),
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TileData {
    data: HashMap<u32, HashMap<u32, TileDesc>>,
    max_x: u32,
    max_y: u32,
}

impl TileData {
    pub fn new(max_x: u32, max_y: u32) -> Self {
        TileData {
            max_x,
            max_y,
            data: HashMap::new(),
        }
    }

    pub fn from_vec(max_x: u32, max_y: u32, tiles: Vec<TileDesc>) -> Result<Self> {
        let mut td = TileData::new(max_x, max_y);
        for t in tiles.into_iter() {
            td.set_tile(t)?;
        }
        Ok(td)
    }

    pub fn set_tilemap_size(&mut self, x: u32, y: u32) {
        self.max_x = x;
        self.max_y = y;
    }

    pub fn set_tile(&mut self, tile: TileDesc) -> Result<()> {
        let (x, y) = tile.coords.into();
        if x > self.max_x || y > self.max_y {
            return Err(TileDataError::OutOfBoundsError(x, y, self.max_x, self.max_y).into());
        }
        let mut col = HashMap::new();
        if let Some(col) = self.data.get_mut(&x) {
            col.insert(y, tile);
        } else {
            col.insert(y, tile);
            self.data.insert(x, col);
        };
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

    pub fn iter(&self) -> TileDataIterator {
        TileDataIterator { td: self, curr_x: 0, curr_y: 0} 
    }
}

pub struct TileDataIterator<'iter> {
    td: &'iter TileData,
    curr_x: u32,
    curr_y: u32,
}

impl<'iter> Iterator for TileDataIterator<'iter> {
    type Item = &'iter TileDesc;

    fn next(&mut self) -> Option<Self::Item> {
        match self.td.data.get(&self.curr_x) {
            Some(col) => {
                self.curr_x = if self.curr_x + 1 > self.td.max_x {
                    0
                } else {
                    self.curr_x + 1
                };

                let val = col.get(&self.curr_y);

                self.curr_y = if self.curr_y + 1 > self.td.max_y {
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
        let tile = td.get_tile(&0, &0)?;
        let (x, y) = tile.coords.into();
        let res = td.get_tile(&3, &3);
        assert_eq!((x, y), (0, 0));
        assert!(res.is_err());
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
        let t2 = td.get_tile(&1, &0)?;

        let (t1x, t1y) = t1.coords.into();
        let (t2x, t2y) = t2.coords.into();
        assert_eq!((t1x, t1y), (0, 0));
        assert_eq!((t2x, t2y), (1, 0));
        Ok(())
    }
}
