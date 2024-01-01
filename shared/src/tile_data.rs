use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::tile_coords::TileCoords;
use crate::tilemap::TileDesc;

#[derive(Debug, Error, PartialEq)]
pub enum TileDataError {
    #[error("Index {0}, {1} exceeds size {2}, {3}")]
    OutOfBoundsError(u32, u32, u32, u32),

    #[error("Tile at {0}, {1} not found")]
    NotFoundError(u32, u32),

    #[error("Unable to initialize blank tilemap: {0}")]
    InitilizationError(String),
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TileData {
    data: HashMap<u32, HashMap<u32, TileDesc>>,
    max_x: u32,
    max_y: u32,
}

impl TileData {
    pub fn new(max_x: u32, max_y: u32) -> Result<Self> {
        let mut td = TileData {
            max_x,
            max_y,
            data: HashMap::with_capacity(max_x as usize),
        };
        td.populate()?;
        Ok(td)
    }

    pub fn from_vec(max_x: u32, max_y: u32, tiles: Vec<TileDesc>) -> Result<Self> {
        let mut td = TileData {
            max_x,
            max_y,
            data: HashMap::with_capacity(max_x as usize),
        };
        td.populate()?;
        for t in tiles.into_iter() {
            td.set_tile(t)?;
        }
        Ok(td)
    }

    fn populate(&mut self) -> Result<()> {
        for x in 0..self.max_x {
            let mut col = HashMap::with_capacity(self.max_y as usize);
            for y in 0..self.max_y {
                let desc = TileDesc::new(None, TileCoords::new(x, y), None);
                col.insert(y, desc);
            }
            self.data.insert(x, col);
        }

        Ok(())
    }

    pub fn len(&self) -> u32 {
        self.max_y * self.max_x
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
            return Err(TileDataError::OutOfBoundsError(*x, *y, self.max_x, self.max_y).into());
        }

        let col = self
            .data
            .get(x)
            .ok_or(TileDataError::NotFoundError(*x, *y))?;
        col.get(y)
            .ok_or(TileDataError::NotFoundError(*x, *y).into())
    }

    // this is broken?
    pub fn iter(&self) -> TileDataIterator {
        TileDataIterator {
            td: self,
            curr_x: 0,
            curr_y: 0,
        }
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
                let val = col.get(&self.curr_y);

                self.curr_y += 1;
                if self.curr_y == self.td.max_y {
                    self.curr_y = 0;
                    self.curr_x += 1;
                }

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
    fn populate() -> Result<()> {
        let td = TileData::new(2, 2)?;
        assert_eq!(4, td.len(), "has 4 items");
        assert!(!td.is_empty(), "is empty");
        let res = td.get_tile(&0, &0);
        assert!(res.is_ok(), "not ok");
        let res = td.get_tile(&1, &1);
        assert!(res.is_ok(), "not ok");
        let res = td.get_tile(&10, &100);
        assert!(res.is_err_and(
            |x| x.downcast_ref() == Some(&TileDataError::OutOfBoundsError(10, 100, 2, 2))
        ));
        Ok(())
    }

    #[test]
    fn set_and_get() -> Result<()> {
        let mut td = TileData::new(2, 2)?;
        td.set_tile(TileDesc::new(Some(0), TileCoords::new(0, 0), None))?;

        let tile = td.get_tile(&0, &0)?;
        let (x, y) = tile.coords.into();
        assert_eq!((x, y), (0, 0));
        assert_eq!(tile.tile_index, Some(0));

        let no_tile = td.get_tile(&1, &1)?;
        let (x, y) = no_tile.coords.into();
        assert_eq!((x, y), (1, 1));
        assert_eq!(no_tile.tile_index, None);
        Ok(())
    }

    #[test]
    fn new_from_vec() -> Result<()> {
        let v = vec![
            TileDesc::new(None, TileCoords::new(0, 0), None),
            TileDesc::new(None, TileCoords::new(1, 0), None),
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

    #[test]
    fn iterator() -> Result<()> {
        let v = vec![
            TileDesc::new(None, TileCoords::new(0, 0), None),
            TileDesc::new(None, TileCoords::new(1, 0), None),
        ];
        let td = TileData::from_vec(2, 2, v)?;
        let mut count = 0;
        let mut x = 0;
        let mut y = 0;
        for t in td.iter() {
            let (n, m) = t.coords.into();
            assert_eq!((n, m), (x, y));
            y += 1;
            if y == 2 {
                y = 0;
                x += 1;
            }
            count += 1;
        }
        assert_eq!(count, 4);
        Ok(())
    }
}
