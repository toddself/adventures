use anyhow::Result;
use bevy::{prelude::*, window::WindowResolution};

use shared::settings::SettingsFile;

fn main() -> Result<()> {
    let settings = SettingsFile::new_from_file("settings.ron")?;

    let editor_height = settings.y_max * settings.tile_height * settings.scale;
    println!("editor height {editor_height}");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1700.0, editor_height),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(settings)
        .add_systems(Startup, (setup_camera, draw_ui))
        .run();
    Ok(())
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn item_rect(builder: &mut ChildBuilder, color: Color, settings: &SettingsFile) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Px(settings.tile_height * settings.scale),
                height: Val::Px(settings.tile_width * settings.scale),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}

fn draw_ui(mut commands: Commands, settings: Res<SettingsFile>) {
        
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![GridTrack::flex(1.0), GridTrack::min_content()],
                grid_template_rows: vec![GridTrack::auto()],
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),
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
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Top content",
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
                        aspect_ratio: Some(1.0),
                        grid_template_columns: RepeatedGridTrack::px(
                            settings.x_max as i32,
                            (settings.tile_width * settings.scale) as f32,
                        ),
                        grid_template_rows: RepeatedGridTrack::px(
                            settings.y_max as i32,
                            (settings.tile_height * settings.scale) as f32,
                        ),
                        row_gap: Val::Px(1.0),
                        column_gap: Val::Px(1.0),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);

                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                    item_rect(builder, Color::ORANGE, &settings);
                    item_rect(builder, Color::BISQUE, &settings);
                    item_rect(builder, Color::BLUE, &settings);
                    item_rect(builder, Color::CRIMSON, &settings);
                });
        });
}
