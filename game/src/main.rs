use anyhow::Result;
use bevy::{prelude::*, sprite::collide_aabb::collide, window::WindowResolution};
use bevy_simple_tilemap::prelude::*;

use shared::components::*;
use shared::settings::{SettingsFile, GameSettings};
use shared::tilemap::{coord_to_screen_pos, MapScreen};

#[derive(Debug, Resource)]
struct MoveTimer(Timer);

fn main() -> Result<()> {
    let sf = SettingsFile::new_from_file("settings.ron")?;

    let settings = GameSettings::new(
        sf.scale,
        sf.x_max,
        sf.y_max,
        sf.input_debounce,
        sf.tile_width,
        sf.tile_height,
        false
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
        .add_systems(Startup, (setup_camera, setup.pipe(error_handler)))
        .add_systems(Update, move_hero)
        .run();
    Ok(())
}

fn error_handler(In(result): In<Result<()>>) {
    if let Err(err) = result {
        println!("encountered an error {:?}", err);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup(
    settings: Res<GameSettings>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> Result<()> {
    // tile map
    let ms = MapScreen::new_from_file("assets/data/test.ron")?;
    commands.spawn(ms.get_tilemap(&settings, &asset_server, texture_atlases));

    // ui
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

    // hero
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("icons/todd.png"),
            transform: Transform {
                translation: coord_to_screen_pos(1, 16, 1.0, &settings),
                scale: Vec3::splat(settings.scale),
                ..default()
            },
            ..Default::default()
        },
        Hero,
    ));

    // walls
    commands.spawn_batch(ms.get_wallmap(&settings));

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
