use anyhow::Result;
use bevy::{prelude::*, window::WindowResolution};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_simple_tilemap::prelude::*;
use uuid;

use shared::settings::{GameSettings, SettingsFile};
use shared::tilemap::MapScreen;

#[derive(Resource, Default)]
struct UiState {
    new_map_dialog_open: bool,
    current_map: MapScreen,
}

fn main() -> Result<()> {
    let sf = SettingsFile::new_from_file("settings.ron")?;
    let settings = GameSettings::new_from_sf(&sf, true);

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
        .init_resource::<UiState>()
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

fn draw_ui(settings: Res<GameSettings>, mut ui_state: ResMut<UiState>, mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    if ui_state.new_map_dialog_open {
        egui::Window::new("Create new map").show(ctx, |ui| {
            let mut new_map_name = String::new();
            let mut tile_source = String::new();
            ui.horizontal_top(|ui| {
                ui.label("map name:");
                ui.text_edit_singleline(&mut new_map_name);
            });
            ui.horizontal_top(|ui| {
                ui.label("map tile:");
                ui.text_edit_singleline(&mut tile_source);
            });
            ui.horizontal_top(|ui| {
                if ui.button("new map").clicked() {
                    ui_state.current_map.map_name = new_map_name.clone();
                    ui_state.current_map.map_id = uuid::Uuid::new_v4();
                    ui_state.current_map.tile_map = tile_source.clone();
                }
                if ui.button("cancel").clicked() {
                    ui_state.new_map_dialog_open = false;
                }
            });
        });
    }

    egui::TopBottomPanel::top("top_panel")
        .default_height(settings.top_margin)
        .show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                if ui.button("new map").clicked() {
                    ui_state.new_map_dialog_open = true;
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
                ui.text_edit_singleline(&mut ui_state.current_map.map_name);
            });
            ui.label(format!("map id: {}", ui_state.current_map.map_id));
            ui.label(format!("tile set: {}", ui_state.current_map.tile_map));
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
    ui_state: Res<UiState>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> Result<()> {
    commands.spawn(ui_state.current_map.get_tilemap(&settings, &asset_server, texture_atlases));
    Ok(())
}
