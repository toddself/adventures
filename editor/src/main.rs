#![forbid(unsafe_code)]
#![deny(future_incompatible)]
#![warn(
    meta_variable_misuse,
    missing_debug_implementations,
    noop_method_call,
    trivial_casts,
    unused_lifetimes,
    unused_macro_rules,
    variant_size_differences
)]
#![doc(test(attr(deny(future_incompatible, rust_2018_idioms, warnings))))]
#![doc(test(attr(allow(unused_extern_crates, unused_variables))))]
#![deny(
    clippy::default_union_representation,
    clippy::exit,
    clippy::lossy_float_literal,
    clippy::mem_forget,
    clippy::multiple_inherent_impl,
    clippy::mut_mut,
    clippy::ptr_as_ptr,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::wildcard_dependencies
)]
#![warn(
    clippy::dbg_macro,
    clippy::empty_drop,
    clippy::fallible_impl_from,
    clippy::inefficient_to_string,
    clippy::macro_use_imports,
    clippy::match_same_arms,
    clippy::no_effect_underscore_binding,
    clippy::panic,
    clippy::print_stderr,
    clippy::same_name_method,
    clippy::single_char_lifetime_names,
    clippy::string_to_string,
    clippy::trait_duplication_in_bounds,
    clippy::type_repetition_in_bounds,
    clippy::unimplemented, // use todo! instead
    clippy::unseparated_literal_suffix,
    clippy::used_underscore_binding,
)]

use std::{env, path::PathBuf};

use anyhow::Result;
use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
    {tasks::AsyncComputeTaskPool, tasks::Task},
};
use bevy_egui::{
    egui::{self, TextureHandle},
    EguiContexts, EguiPlugin,
};
use bevy_simple_tilemap::prelude::*;
use futures_lite::future;
use rfd::FileDialog;
use thiserror::Error;

use shared::tilemap::{MapScreen, TileDesc};
use shared::{
    settings::{GameSettings, SettingsFile},
    tile_coords::{top_left_to_coord, TileCoords},
};

#[derive(Resource, Default)]
struct UiState {
    current_map: MapScreen,
    tile_handles: Option<Vec<egui::TextureHandle>>,
    tile_source: Option<PathBuf>,
    tile_size: [usize; 2],
    selected_tile: Option<TextureHandle>,
    selected_tile_index: Option<u32>,
    cursor_pos: Option<Vec2>,
    current_tile: Option<TileCoords>,
}

#[derive(Resource, Default)]
struct FileDialogState {
    chosen_file: Option<PathBuf>,
    error_message: Option<String>,
    dialog_open: bool,
    new_map: bool,
}

#[derive(Component)]
struct SelectedFile(Task<Option<PathBuf>>);

#[derive(Debug, Error)]
enum SystemError {
    #[error("Unable to generate tilemap")]
    BadTilemap,
}

fn main() -> Result<()> {
    let settings_file = env::var("CONFIG_FILE").unwrap_or("settings.ron".to_string());
    let sf = SettingsFile::new_from_file(&settings_file)?;
    let settings = GameSettings::new_from_sf(&sf, true);

    println!(
        "Creating editor: {}x{}",
        settings.viewport_width, settings.viewport_height
    );

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            settings.viewport_width,
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
        .add_systems(
            Startup,
            (
                initialize_scene.pipe(error_handler),
                // draw_map.pipe(error_handler),
            ),
        )
        .add_systems(
            Update,
            (
                poll_file_dialog,
                draw_ui.pipe(error_handler),
                mouse_button_input.pipe(error_handler),
                draw_map.pipe(error_handler),
            ),
        )
        .run();
    Ok(())
}

fn initialize_scene(
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
    settings: Res<GameSettings>,
) -> Result<()> {
    commands.spawn(Camera2dBundle::default());
    ui_state.tile_size = [16, 16];
    ui_state.current_map.tile_data.set_tilemap_size(
        settings.game_area_tile_x_max.floor() as u32,
        settings.game_area_tile_y_max.floor() as u32,
    )?;
    Ok(())
}

fn error_handler(In(result): In<Result<()>>) {
    if let Err(err) = result {
        error!("encountered an error {:?}", err);
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

    if fds.new_map {
        if let Some(texture_path) = &ui_state.tile_source {
            let tile_map_image = load_image_from_path(texture_path)?;
            let tile_map_texture = ctx.load_texture(
                "tile_map_texture",
                tile_map_image.clone(),
                Default::default(),
            );
            let tile_map_size = tile_map_texture.size();
            let rows = tile_map_size[0] / ui_state.tile_size[0];
            let cols = tile_map_size[1] / ui_state.tile_size[1];
            let mut handles = vec![];
            let mut tile_index = 0;
            for row in 0..rows {
                for col in 0..cols {
                    let left = row * ui_state.tile_size[0];
                    let top = col * ui_state.tile_size[1];
                    let right = left + ui_state.tile_size[0];
                    let bottom = top + ui_state.tile_size[1];
                    let rect = egui::Rect {
                        min: egui::pos2(left as f32, top as f32),
                        max: egui::pos2(right as f32, bottom as f32),
                    };
                    let handle = ctx.load_texture(
                        format!("{tile_index}"),
                        tile_map_image.region(&rect, None),
                        Default::default(),
                    );
                    handles.push(handle);
                    tile_index += 1;
                }
            }
            ui_state.tile_handles = Some(handles);
        }
        fds.new_map = false;
    }

    if fds.dialog_open {
        egui::Window::new("Create new map").show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.label("map name:");
                ui.text_edit_singleline(&mut ui_state.current_map.map_name);
            });
            ui.horizontal_top(|ui| {
                let tile_map_name = match &fds.chosen_file {
                    Some(tm) => tm.to_string_lossy().into_owned(),
                    None => "".to_owned(),
                };
                ui.label(format!("map tile: {tile_map_name}"));
                if ui.button("select file").clicked() {
                    let dir = std::env::current_dir().unwrap_or("/".into());
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
                    ui.label(egui::RichText::new(error_message).color(egui::Color32::RED))
                });
            }
            ui.horizontal_top(|ui| {
                if ui.button("new map").clicked() {
                    ui_state.current_map.map_id = uuid::Uuid::new_v4();
                    if let Some(map_file) = &fds.chosen_file {
                        ui_state.current_map.tile_set = Some(map_file.to_path_buf());
                        ui_state.tile_source = Some(map_file.to_path_buf());
                        fds.dialog_open = false;
                        fds.chosen_file = None;
                        fds.error_message = None;
                        fds.new_map = true;
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
            let tile_map_name = match &ui_state.current_map.tile_set {
                Some(tm) => tm.to_string_lossy().into_owned(),
                None => "".to_owned(),
            };
            ui.label(format!("tile set: {tile_map_name}"));
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });

    let side_panel_frame = egui::containers::Frame {
        fill: egui::Color32::DARK_GRAY,
        ..Default::default()
    };

    egui::SidePanel::left("side_panel")
        .frame(side_panel_frame)
        .resizable(false)
        .default_width(settings.left_margin)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let tm_name = match &ui_state.tile_source {
                    Some(ts) => ts
                        .to_string_lossy()
                        .into_owned()
                        .split('/')
                        .last()
                        .unwrap_or("")
                        .to_owned(),
                    None => "No tile source selected".to_owned(),
                };
                ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::TOP).with_main_justify(true),
                    |ui| {
                        ui.label(
                            egui::RichText::new(tm_name)
                                .size(16.)
                                .strong()
                                .color(egui::Color32::BLACK),
                        )
                    },
                );

                ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::TOP).with_main_wrap(true),
                    |ui| {
                        ui.style_mut().spacing.item_spacing = egui::vec2(10., 10.);

                        ui.style_mut().visuals.widgets.active = egui::style::WidgetVisuals {
                            bg_fill: egui::Color32::GOLD,
                            weak_bg_fill: egui::Color32::GOLD,
                            bg_stroke: egui::Stroke::NONE,
                            rounding: egui::Rounding::ZERO,
                            fg_stroke: egui::Stroke::NONE,
                            expansion: 0.,
                        };

                        ui.style_mut().visuals.widgets.inactive = egui::style::WidgetVisuals {
                            bg_fill: egui::Color32::DARK_GRAY,
                            weak_bg_fill: egui::Color32::DARK_GRAY,
                            bg_stroke: egui::Stroke::NONE,
                            rounding: egui::Rounding::ZERO,
                            fg_stroke: egui::Stroke::NONE,
                            expansion: 0.,
                        };

                        ui.style_mut().visuals.window_fill = egui::Color32::RED;

                        if let Some(tile_handles) = ui_state.tile_handles.clone() {
                            tile_handles.iter().for_each(|h| {
                                let size = h.size_vec2();
                                let scaled =
                                    egui::vec2(size.x * settings.scale, size.y * settings.scale);

                                let tilemap_button = egui::widgets::ImageButton::new(
                                    egui::load::SizedTexture::new(h.id(), scaled),
                                )
                                .selected(Some(h) == ui_state.selected_tile.as_ref())
                                .frame(true);

                                if ui.add(tilemap_button).clicked() {
                                    bevy::log::trace!("clicked on {:?}", h.name());
                                    ui_state.selected_tile = Some(h.clone());
                                    ui_state.selected_tile_index =
                                        Some(h.name().parse().unwrap_or(0));
                                }
                            })
                        }
                    },
                );
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            });
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
    if ui_state.current_map.tile_set.is_some() {
        bevy::log::debug!("in draw map with a tile_set");
        let tilemap = ui_state
            .current_map
            .get_tilemap(&settings, &asset_server, texture_atlases)?;
        commands.spawn(tilemap);
    }
    Ok(())
}

fn mouse_button_input(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<Input<MouseButton>>,
    settings: Res<GameSettings>,
    mut ui_state: ResMut<UiState>,
) -> Result<()> {
    let position = q_windows.single().cursor_position();

    if let Some(pos) = position {
        if Some(pos) != ui_state.cursor_pos {
            ui_state.cursor_pos = Some(pos);
            let screen_pos = Vec3::new(pos.x, pos.y, 0.);
            ui_state.current_tile = top_left_to_coord(screen_pos, &settings);
            bevy::log::trace!(
                "absolute cursor: {:?}, current tile is {:?}",
                ui_state.cursor_pos,
                ui_state.current_tile
            );
        }
    }

    if buttons.just_pressed(MouseButton::Left) && ui_state.current_tile.is_some() {
        bevy::log::trace!(
            "pressed left mouse button at {:?}, tile: {:?}, will paint index {:?}",
            ui_state.cursor_pos,
            ui_state.current_tile,
            ui_state.selected_tile_index
        );

        // TODO: this is not working
        if let Some(coords) = &ui_state.current_tile.clone() {
            bevy::log::debug!("setting coords {coords}");
            if let Some(tile_index) = &ui_state.selected_tile_index {
                let tile = TileDesc::new(Some(*tile_index), *coords, None);
                ui_state.current_map.tile_data.set_tile(tile)?;
            }
        };
    } else if buttons.just_pressed(MouseButton::Right) {
        bevy::log::debug!(
            "pressed right mouse button at {:?}, tile: {:?}",
            ui_state.cursor_pos,
            ui_state.current_tile
        );
    }

    Ok(())
}
