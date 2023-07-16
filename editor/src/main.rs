use bevy::{prelude::*, window::WindowResolution};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                                        1700.0,
                                        1110.0
                        ),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, (setup_camera, draw_ui))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw_ui(mut commands: Commands) {

}
