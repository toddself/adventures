use anyhow::Result;
use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_simple_tilemap::prelude::*;
use uuid;

use shared::settings::{GameSettings, SettingsFile};
use shared::tilemap::{MapScreen, TileType};

#[derive(Resource)]
struct NoTile(Option<Handle<Image>>);

#[derive(Component)]
struct EditableTile;

#[derive(Resource, Default, Debug)]
struct EditorState {
    map_name: String,
    map_id: uuid::Uuid,
    tile_source: String,
}

// origin for x,y is lower left
#[derive(Default, Debug)]
struct EditorTile {
    x: usize,
    y: usize,
    metadata: Vec<TileType>,
    tile_index: usize,
}

fn main() -> Result<()> {
    let sf = SettingsFile::new_from_file("settings.ron")?;
    let settings = GameSettings::new(
        sf.scale,
        sf.x_max,
        sf.y_max,
        sf.input_debounce,
        sf.tile_width,
        sf.tile_height,
        true,
    );

    let state = EditorState::default();

    println!(
        "Creating editor: {}x{}",
        settings.editor_viewport_width, settings.viewport_height
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
        .add_plugins((EguiPlugin, SimpleTileMapPlugin))
        .insert_resource(settings)
        .insert_resource(state)
        .insert_resource(NoTile(None))
        .add_systems(Startup, (setup_camera, draw_map.pipe(error_handler)))
        .add_systems(Update, draw_ui)
        .run();
    Ok(())
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn error_handler(In(result): In<Result<()>>) {
    if let Err(err) = result {
        println!("encountered an error {:?}", err);
    }
}

fn draw_ui(
    settings: Res<GameSettings>,
    mut editor_state: ResMut<EditorState>,
    mut contexts: EguiContexts,
) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("top_panel")
        .default_height(settings.top_margin)
        .show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                if ui.button("new map").clicked() {
                    info!("new map");
                }
                if ui.button("save map").clicked() {
                    info!("save map");
                }
                if ui.button("load map").clicked() {
                    info!("load map");
                }
            });
            ui.horizontal_top(|ui| {
                ui.label("map name");
                ui.text_edit_singleline(&mut editor_state.map_name);
            });
            ui.label(format!("map id: {}", editor_state.map_id));
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });

    egui::SidePanel::left("side_panel")
        .default_width(settings.left_margin)
        .show(ctx, |ui| {
            ui.heading("side panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });
}

fn draw_map(
    settings: Res<GameSettings>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> Result<()> {
    let ms = MapScreen::new_from_file("assets/data/test.ron")?;
    commands.spawn(ms.get_tilemap(&settings, &asset_server, texture_atlases));
    Ok(())
}
