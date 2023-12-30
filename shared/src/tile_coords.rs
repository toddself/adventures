use std::fmt::Display;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::settings::GameSettings;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TileCoords(u32, u32);

impl Display for TileCoords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.0, self.1)
    }
}

impl TileCoords {
    pub fn new(x: u32, y: u32) -> Self {
        TileCoords(x, y)
    }

    pub fn x(&self) -> u32 {
        self.0
    }

    pub fn y(&self) -> u32 {
        self.1
    }
}

impl From<TileCoords> for (u32, u32) {
    fn from(value: TileCoords) -> Self {
        (value.x(), value.y())
    }
}

// converts a coordinate (origin bottom left) to an screen position, (origin top left)
pub fn coord_to_screen_pos(x: u32, y: u32, z: f32, settings: &GameSettings) -> Vec3 {
    let new_x = (x as f32 * settings.tile_width * settings.scale) + settings.game_area_x_transform;
    let new_y = (y as f32 * settings.tile_height * settings.scale) + settings.game_area_y_transform;
    Vec3::new(new_x, new_y, z)
}

pub fn screen_pos_to_coord(coords: Vec3, settings: &GameSettings) -> Option<TileCoords> {
    let tile_x = settings.tile_width * settings.scale;
    let tile_y = settings.tile_height * settings.scale;

    let new_x = ((settings.game_area_x_transform - coords.x) * -1.) / tile_x;
    let new_y = ((settings.game_area_y_transform - coords.y) * -1.) / tile_y;

    if new_x >= 0. && new_y >= 0. {
        Some(TileCoords(new_x as u32, new_y as u32))
    } else {
        None
    }
}

pub fn top_left_to_coord(coords: Vec3, settings: &GameSettings) -> Option<TileCoords> {
    let y = (coords.y - settings.viewport_height).abs();
    let x = coords.x - settings.left_margin;

    let tile_x = settings.tile_width * settings.scale;
    let tile_y = settings.tile_height * settings.scale;

    let new_x = x / tile_x;
    let new_y = y / tile_y;

    if new_x >= 0. && new_y >= 0. {
        Some(TileCoords(new_x as u32, new_y as u32))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::settings::SettingsFile;
    use anyhow::Result;

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
        assert_eq!(pos0, Some(TileCoords(0, 0)));
        let pos1 = screen_pos_to_coord(
            Vec3::new(
                gs.game_area_x_transform + (gs.scale * gs.tile_width),
                gs.game_area_y_transform + (gs.scale * gs.tile_height),
                0.0,
            ),
            &gs,
        );
        assert_eq!(pos1, Some(TileCoords(1, 1)));

        // anywhere in tile selects proper tile
        let pos2 = screen_pos_to_coord(
            Vec3::new(
                gs.game_area_x_transform + (gs.scale * gs.tile_width * 4. + 2.),
                gs.game_area_y_transform + (gs.scale * gs.tile_height * 4. + 2.),
                0.0,
            ),
            &gs,
        );
        assert_eq!(pos2, Some(TileCoords(4, 4)));
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
        assert_eq!(pos0, Some(TileCoords(0, 0)));
        let pos1 = screen_pos_to_coord(
            Vec3::new(
                gs.game_area_x_transform + (gs.scale * gs.tile_width),
                gs.game_area_y_transform + (gs.scale * gs.tile_height),
                0.0,
            ),
            &gs,
        );
        assert_eq!(pos1, Some(TileCoords(1, 1)));

        // anywhere in tile selects proper tile
        let pos2 = screen_pos_to_coord(
            Vec3::new(
                gs.game_area_x_transform + (gs.scale * gs.tile_width * 4. + 2.),
                gs.game_area_y_transform + (gs.scale * gs.tile_height * 4. + 2.),
                0.0,
            ),
            &gs,
        );
        assert_eq!(pos2, Some(TileCoords(4, 4)));
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
        assert_eq!(pos0, Some(TileCoords(0, 0)));

        let screen_pos = Vec3::new(gs.left_margin + 100., gs.viewport_height - 100., 0.);
        let pos0 = top_left_to_coord(screen_pos, &gs);
        assert_eq!(pos0, Some(TileCoords(6, 6)));
        Ok(())
    }
}
