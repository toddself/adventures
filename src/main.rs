use bevy::{
    math::{ivec3, vec2},
    prelude::*,
    window::WindowResolution,
};
use bevy_simple_tilemap::prelude::*;

use serde::{Deserialize, Serialize};

// 0,0 should be lower left for fuckssake
const SCALE: f32 = 343.0;
const TILE_X_MAX: f32 = 24.0;
const TILE_Y_MAX: f32 = 18.0;
const GAME_HEIGHT: f32 = 1400.0;
const GAME_WIDTH: f32 = 2000.0;

const TILE_X: f32 = 16.0;
const TILE_Y: f32 = 16.0;
const TILE_MAP_ROWS: u32 = 16;
const TILE_MAP_COLS: u32 = 16;
const X_RES: f32 = TILE_X_MAX * (TILE_X * SCALE);
const Y_RES: f32 = TILE_Y_MAX * (TILE_Y * SCALE);
const LEFT_MARGIN: f32 = GAME_WIDTH - X_RES;
const TOP_MARGIN: f32 = GAME_HEIGHT - Y_RES;
const XPT: f32 = (X_RES / 2.0) * -1.0 + (TILE_X * SCALE / 2.0) + LEFT_MARGIN / 2.0;
const YPT: f32 = (Y_RES / 2.0) * -1.0 + (TILE_Y * SCALE / 2.0) - TOP_MARGIN / 2.0;

#[derive(Debug, Serialize, Deserialize)]
struct MapScreen {
    tile_map: String,
    data: Vec<TileDesc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TileDesc {
    tile_source: (u32, u32),
    screen_pos: (i32, i32, i32),
}

impl From<&TileDesc> for (IVec3, Option<Tile>) {
    fn from(t: &TileDesc) -> (IVec3, Option<Tile>) {
        let sprite_index = t.tile_source.1 + (t.tile_source.0 * TILE_MAP_ROWS);
        println!("selecting tile {sprite_index}");
        let tile = Tile {
            sprite_index,
            ..default()
        };
        let v3 = ivec3(t.screen_pos.0, t.screen_pos.1, t.screen_pos.2);
        (v3, Some(tile))
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(GAME_WIDTH, GAME_HEIGHT),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(SimpleTileMapPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let d = std::fs::read_to_string("assets/data/map1.ron").expect("nah");
    let ms: MapScreen = ron::from_str(&d).unwrap();

    let texture_handle = asset_server.load(ms.tile_map);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        vec2(TILE_X, TILE_Y),
        TILE_MAP_COLS as usize,
        TILE_MAP_ROWS as usize,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut tilemap = TileMap::default();
    let map_data: Vec<(IVec3, Option<Tile>)> = ms.data.iter().map(|x| x.into()).collect();
    tilemap.set_tiles(map_data);

    let tilemap_bundle = TileMapBundle {
        tilemap,
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform {
            translation: Vec3::new(XPT, YPT, 1.0),
            scale: Vec3::splat(SCALE),
            ..default()
        },
        ..default()
    };

    commands.spawn(Camera2dBundle::default());
    commands.spawn(tilemap_bundle);
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(TOP_MARGIN),
                        top: Val::Px(0.0),
                        left: Val::Px(0.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "top test",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(LEFT_MARGIN),
                        height: Val::Px(GAME_HEIGHT - TOP_MARGIN),
                        top: Val::Px(TOP_MARGIN),
                        left: Val::Px(0.0),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "left test",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });
        });
}
