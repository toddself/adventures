use std::fs;

use anyhow::Result;
use bevy::{math::vec2, prelude::*, sprite::collide_aabb::collide, window::WindowResolution};
use bevy_simple_tilemap::prelude::*;
use serde::{Deserialize, Serialize};

mod settings;
mod tilemap;
use settings::GameSettings;
use tilemap::MapScreen;

#[derive(Debug, Component)]
struct Hero;

#[derive(Debug, Resource)]
struct MoveTimer(Timer);

#[derive(Debug, Component)]
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

    let settings = GameSettings::new(
        sf.scale,
        sf.x_max,
        sf.y_max,
        sf.input_debounce,
        sf.tile_width,
        sf.tile_height,
    );
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            settings.game_area_x_res,
                            settings.viewport_height,
                        ),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(SimpleTileMapPlugin)
        .insert_resource(settings)
        .insert_resource(MoveTimer(Timer::from_seconds(
            sf.input_debounce,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup.pipe(error_handler))
        .add_systems(Update, move_hero)
        .run();
    Ok(())
}

fn error_handler(In(result): In<Result<()>>) {
    if let Err(err) = result {
        println!("encountered an error {:?}", err);
    }
}

fn setup(
    settings: Res<GameSettings>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> Result<()> {
    let ms = MapScreen::new_from_file("assets/data/map1.ron")?;

    let texture_handle = asset_server.load(&ms.tile_map);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        vec2(settings.tile_width, settings.tile_height),
        ms.tile_cols as usize,
        ms.tile_rows as usize,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut tilemap = TileMap::default();
    tilemap.set_tiles(ms.tilemap_from_struct());

    let tilemap_bundle = TileMapBundle {
        tilemap,
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform {
            translation: Vec3::new(
                settings.game_area_x_transform,
                settings.game_area_y_transform,
                0.0,
            ),
            scale: Vec3::splat(settings.scale),
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
                        height: Val::Px(settings.top_margin),
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
                translation: Vec3::new(
                    settings.game_area_x_transform,
                    settings.game_area_y_transform,
                    1.0,
                ),
                scale: Vec3::splat(settings.scale),
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
                    settings.game_area_x_transform + (settings.tile_width * settings.scale),
                    settings.game_area_y_transform + (settings.tile_height * settings.scale),
                    1.0,
                ),
                scale: Vec3::splat(settings.scale),
                ..default()
            },
            ..Default::default()
        },
        Wall,
    ));

    Ok(())
}

fn move_hero(
    settings: Res<GameSettings>,
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

        let new_x =
            hero_transform.translation.x + direction.0 * (settings.tile_width * settings.scale);
        let new_y =
            hero_transform.translation.y + direction.1 * (settings.tile_height * settings.scale);

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

            if collision.is_some() {
                return;
            }
        }

        hero_transform.translation.x =
            new_x.clamp(settings.game_area_x_min, settings.game_area_x_max);
        hero_transform.translation.y =
            new_y.clamp(settings.game_area_y_min, settings.game_area_y_max);
    }
}
