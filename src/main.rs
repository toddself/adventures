use std::fs;

use anyhow::Result;

use bevy::{
    math::{ivec3, vec2},
    prelude::*,
    sprite::collide_aabb::collide,
    window::WindowResolution,
};
// use bevy_rapier2d::prelude::*;
use bevy_simple_tilemap::prelude::*;

use serde::{Deserialize, Serialize};

mod settings;
use settings::GameSettings;

const TILE_MAP_ROWS: u32 = 16;
const TILE_MAP_COLS: u32 = 16;

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
        let tile = Tile {
            sprite_index,
            ..default()
        };
        let v3 = ivec3(t.screen_pos.0, t.screen_pos.1, t.screen_pos.2);
        (v3, Some(tile))
    }
}

#[derive(Component)]
struct Hero;

#[derive(Resource)]
struct MoveTimer(Timer);

#[derive(Component)]
struct Wall;

#[derive(Debug, Serialize, Deserialize)]
struct SettingsFile {
    scale: f32,
    x_max: f32,
    y_max: f32,
    input_debounce: f32,
    tile_width: f32,
    tile_height: f32,
}

fn main() -> Result<()> {
    let settings_data = fs::read_to_string("settings.ron")?;
    let sf: SettingsFile = ron::from_str(&settings_data)?;

    let settings = GameSettings::new(sf.scale, sf.x_max, sf.y_max, sf.input_debounce, sf.tile_width, sf.tile_height);
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(settings.game_area_x_res, settings.viewport_height),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(SimpleTileMapPlugin)
        .insert_resource(MoveTimer(Timer::from_seconds(
            INPUT_DEBOUNCE,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_hero)
        .run();
    Ok(())
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
        vec2(TILE_WIDTH, TILE_HEIGHT),
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
            translation: Vec3::new(GAME_AREA_X_TRANSFORM, GAME_AREA_Y_TRANSFORM, 0.0),
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
                        "HARRY HAS NO HEALTH",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });
        });

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("icons/todd.png"),
            transform: Transform {
                translation: Vec3::new(GAME_AREA_X_TRANSFORM, GAME_AREA_Y_TRANSFORM, 1.0),
                scale: Vec3::splat(SCALE),
                ..default()
            },
            ..Default::default()
        },
        Hero,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("icons/wall.png"),
            transform: Transform {
                translation: Vec3::new(
                    GAME_AREA_X_TRANSFORM + (TILE_WIDTH * SCALE),
                    GAME_AREA_Y_TRANSFORM + (TILE_HEIGHT * SCALE),
                    1.0,
                ),
                scale: Vec3::splat(SCALE),
                ..default()
            },
            ..Default::default()
        },
        Wall,
    ));
}

fn move_hero(
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut hero_query: Query<&mut Transform, With<Hero>>,
    wall_query: Query<&Transform, (With<Wall>, Without<Hero>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut hero_transform = hero_query.single_mut();
        let mut direction = (0.0, 0.0);

        if keyboard_input.pressed(KeyCode::Left) {
            direction = (-1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction = (1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction = (0.0, 1.0);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction = (0.0, -1.0);
        }

        let new_x = hero_transform.translation.x + direction.0 * (TILE_WIDTH * SCALE);
        let new_y = hero_transform.translation.y + direction.1 * (TILE_HEIGHT * SCALE);

        let hero_size = hero_transform.scale.truncate();

        for transform in &wall_query {
            let collision = collide(
                Vec3 {
                    x: new_x,
                    y: new_y,
                    z: 1.0,
                },
                hero_size,
                transform.translation,
                transform.scale.truncate(),
            );

            println!("collision: {:?}", collision);

            if collision.is_some() {
                return;
            }
        }

        hero_transform.translation.x = new_x.clamp(GAME_AREA_X_MIN, GAME_AREA_X_MAX);
        hero_transform.translation.y = new_y.clamp(GAME_AREA_Y_MIN, GAME_AREA_Y_MAX);
    }
}
