use anyhow::Result;
use bevy::{input::keyboard::KeyboardInput, prelude::*, window::WindowResolution};

use shared::settings::{GameSettings, SettingsFile};

#[derive(Resource)]
struct NoTile(Option<Handle<Image>>);

#[derive(Component)]
struct EditableTile;

fn main() -> Result<()> {
    let sf = SettingsFile::new_from_file("settings.ron")?;
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
                            settings.editor_viewport_width,
                            settings.viewport_height,
                        ),
                        title: "Lazy Cat Game Editor".to_owned(),
                        ime_enabled: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(settings)
        .insert_resource(NoTile(None))
        .add_systems(Startup, (setup_camera, load_assets, draw_ui).chain())
        .run();
    Ok(())
}

fn load_assets(asset_server: Res<AssetServer>, mut no_tile: ResMut<NoTile>) {
    let no_tile_handle: Handle<Image> = asset_server.load("tiles/no-tile.png");
    *no_tile = NoTile(Some(no_tile_handle));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn map_tile(builder: &mut ChildBuilder, texture: &Option<Handle<Image>>, settings: &GameSettings) {
    let width = settings.tile_width * settings.scale;
    let height = settings.tile_height * settings.scale;
    info!("width: {width}, height: {height}");

    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                margin: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: Color::BISQUE.into(),
            ..default()
        })
        .with_children(|builder| {
            match texture {
                Some(texture) => builder.spawn((
                    ImageBundle {
                        image: texture.clone().into(),
                        transform: Transform {
                            scale: Vec3::splat(settings.scale),
                            ..default()
                        },
                        ..default()
                    },
                    EditableTile,
                )),
                None => builder.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        transform: Transform {
                            scale: Vec3::splat(settings.scale),
                            ..default()
                        },
                        ..default()
                    },
                    EditableTile,
                )),
            };
        });
}

fn draw_ui(mut commands: Commands, settings: Res<GameSettings>) {
    let width = settings.tile_width * settings.scale;
    let height = settings.tile_height * settings.scale;
    info!("width: {width}, height: {height}");

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::min_content()],
                grid_template_rows: vec![GridTrack::auto()],
                ..default()
            },
            background_color: Color::GRAY.into(),
            ..default()
        })
        .with_children(|builder| {
            // header
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_row: GridPlacement::span(settings.x_max as u16),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: Color::DARK_GRAY.into(),
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "left content",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });

            // grid
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                        aspect_ratio: Some(1.0),
                        grid_template_columns: RepeatedGridTrack::auto(settings.x_max as u16),
                        grid_template_rows: RepeatedGridTrack::auto(settings.y_max as u16),
                        row_gap: Val::Px(1.0),
                        column_gap: Val::Px(1.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    for _x in 0..settings.x_max as usize {
                        for _y in 0..settings.y_max as usize {
                            map_tile(builder, &None, &settings)
                        }
                    }
                });
        });
}
