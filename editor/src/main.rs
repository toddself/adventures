use std::path::PathBuf;

use anyhow::Result;
use bevy::{
    prelude::*,
    window::WindowResolution,
    {tasks::AsyncComputeTaskPool, tasks::Task},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::egui::Color32;
use bevy_simple_tilemap::prelude::*;
use futures_lite::future;
use rfd::FileDialog;
use simple_home_dir::home_dir;
use uuid;

use shared::settings::{GameSettings, SettingsFile};
use shared::tilemap::MapScreen;

#[derive(Resource, Default)]
struct UiState {
    current_map: MapScreen,
    tile_texture_handle: Option<egui::TextureHandle>,
    tile_source: Option<PathBuf>,
}

#[derive(Resource, Default)]
struct FileDialogState {
    chosen_file: Option<PathBuf>,
    error_message: Option<String>,
    dialog_open: bool,
}

#[derive(Component)]
struct SelectedFile(Task<Option<PathBuf>>);

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
        .init_resource::<FileDialogState>()
        .init_resource::<UiState>()
        .add_systems(Startup, (setup_camera, draw_map.pipe(error_handler)))
        .add_systems(Update, (poll_file_dialog, draw_ui.pipe(error_handler)))
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

// using the file dialog is a slog!
fn poll_file_dialog(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut SelectedFile)>,
    mut fds: ResMut<FileDialogState>,
) {
    for (entity, mut selected_file) in tasks.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut selected_file.0)) {
            fds.chosen_file = result;
            commands.entity(entity).remove::<SelectedFile>();
        }
    }
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

fn draw_ui(
    mut commands: Commands,
    settings: Res<GameSettings>,
    mut ui_state: ResMut<UiState>,
    mut fds: ResMut<FileDialogState>,
    mut contexts: EguiContexts,
) -> Result<()> {
    let ctx = contexts.ctx_mut();

    if let Some(texture_path) = &ui_state.tile_source {
        let img = load_image_from_path(&texture_path)?;
        ui_state
            .tile_texture_handle
            .get_or_insert_with(|| ctx.load_texture("tile_texture", img, Default::default()));
    }

    if fds.dialog_open {
        egui::Window::new("Create new map").show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.label("map name:");
                ui.text_edit_singleline(&mut ui_state.current_map.map_name);
            });
            ui.horizontal_top(|ui| {
                ui.label(format!("map tile: {:?}", fds.chosen_file));
                if ui.button("select file").clicked() {
                    let dir = match home_dir() {
                        Some(p) => p,
                        None => "/".into(),
                    };
                    let thread_pool = AsyncComputeTaskPool::get();
                    let task = thread_pool.spawn(async move {
                        FileDialog::new()
                            .add_filter("images", &["png", "gif", "jpg"])
                            .set_directory(dir)
                            .pick_file()
                    });
                    commands.spawn(SelectedFile(task));
                }
            });
            if let Some(error_message) = &fds.error_message {
                ui.horizontal_top(|ui| {
                    ui.label(egui::RichText::new(error_message).color(Color32::RED))
                });
            }
            ui.horizontal_top(|ui| {
                if ui.button("new map").clicked() {
                    ui_state.current_map.map_id = uuid::Uuid::new_v4();
                    if let Some(map_file) = &fds.chosen_file {
                        ui_state.current_map.tile_map = map_file.to_path_buf();
                        ui_state.tile_source = Some(map_file.to_path_buf());
                        fds.dialog_open = false;
                        fds.chosen_file = None;
                        fds.error_message = None;
                    } else {
                        fds.chosen_file = None;
                        fds.error_message = Some(String::from("No valid file was chosen!"));
                    }
                }
                if ui.button("cancel").clicked() {
                    fds.dialog_open = false;
                }
            });
        });
    }

    egui::TopBottomPanel::top("top_panel")
        .default_height(settings.top_margin)
        .show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                if ui.button("new map").clicked() {
                    fds.dialog_open = true;
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
            ui.label(format!("tile set: {:?}", ui_state.current_map.tile_map));
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });

    egui::SidePanel::left("side_panel")
        .default_width(settings.left_margin)
        .show(ctx, |ui| {
            ui.heading("side panel");
            if let Some(texture_handle) = &ui_state.tile_texture_handle {
                ui.add(egui::widgets::Image::new(
                    texture_handle.id(),
                    texture_handle.size_vec2(),
                ));
            }
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });

    Ok(())
}

fn draw_map(
    settings: Res<GameSettings>,
    ui_state: Res<UiState>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> Result<()> {
    commands.spawn(
        ui_state
            .current_map
            .get_tilemap(&settings, &asset_server, texture_atlases),
    );
    Ok(())
}
