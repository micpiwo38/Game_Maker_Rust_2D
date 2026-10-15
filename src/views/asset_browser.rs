use std::collections::HashMap;
use std::fs;
use std::path::Path;
use egui::{Color32, TextureHandle, Rect};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default)]
pub struct AssetEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub sub_sprites: Vec<AssetEntry>,
    pub uv_rect: Option<[f32; 4]>, // Coordonnées UV pour découper la portion de la spritesheet
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SpriteMeta {
    pub cols: usize,
    pub rows: usize,
}

pub fn load_directory(path: &str) -> Vec<AssetEntry> {
    let mut entries = Vec::new();

    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".meta") {
                        continue;
                    }

                    let file_path = entry.path().to_string_lossy().into_owned();
                    let mut sub_sprites = Vec::new();

                    if file_name.to_lowercase().ends_with(".png") {
                        let meta_path = format!("{}.meta", file_path);
                        if let Ok(meta_content) = fs::read_to_string(&meta_path) {
                            if let Ok(meta) = serde_json::from_str::<SpriteMeta>(&meta_content) {
                                let total_cols = meta.cols as f32;
                                let total_rows = meta.rows as f32;
                                let total_cuts = meta.cols * meta.rows;

                                for i in 0..total_cuts {
                                    let c = (i % meta.cols) as f32;
                                    let r = (i / meta.cols) as f32;

                                    let min_x = c / total_cols;
                                    let min_y = r / total_rows;
                                    let max_x = (c + 1.0) / total_cols;
                                    let max_y = (r + 1.0) / total_rows;

                                    sub_sprites.push(AssetEntry {
                                        name: format!("{}_sub_{}", file_name, i),
                                        path: file_path.clone(),
                                        is_dir: false,
                                        is_expanded: false,
                                        sub_sprites: vec![],
                                        uv_rect: Some([min_x, min_y, max_x, max_y]),
                                    });
                                }
                            }
                        }
                    }

                    entries.push(AssetEntry {
                        name: file_name.to_string(),
                        path: file_path,
                        is_dir: file_type.is_dir(),
                        is_expanded: false,
                        sub_sprites,
                        uv_rect: None,
                    });
                }
            }
        }
    }
    entries
}

pub fn load_texture_from_file(ctx: &egui::Context, path: &str) -> Option<TextureHandle> {
    let image_bytes = fs::read(path).ok()?;
    let dynamic_image = image::load_from_memory(&image_bytes).ok()?;
    let rgba_image = dynamic_image.to_rgba8();
    let width = rgba_image.width() as usize;
    let height = rgba_image.height() as usize;
    let pixels = rgba_image.into_raw();
    let egui_image = egui::ColorImage::from_rgba_unmultiplied([width, height], &pixels);
    Some(ctx.load_texture(path, egui_image, egui::TextureOptions::NEAREST))
}

pub fn render(
    ctx: &egui::Context,
    current_path: &mut String,
    assets: &mut [AssetEntry],
    texture_cache: &mut HashMap<String, TextureHandle>,
    selected_asset: &mut Option<AssetEntry>,
    selected_object: &mut Option<String>
) {
    egui::TopBottomPanel::bottom("asset_browser_panel")
        .default_height(220.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Project / Assets");
                ui.separator();
                ui.label(format!("Path: {}", current_path));

                if current_path != "assets" && ui.button("⬆ Back").clicked() {
                    let path = Path::new(current_path);
                    if let Some(parent) = path.parent() {
                        *current_path = parent.to_string_lossy().into_owned();
                    }
                }
            });
            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if assets.is_empty() {
                        ui.colored_label(Color32::LIGHT_RED, "Folder is empty !");
                        return;
                    }

                    let available_width = ui.available_width();
                    let item_width = 100.0;
                    let columns = (available_width / item_width).floor().max(1.0) as usize;

                    for chunk in assets.chunks_mut(columns) {
                        ui.horizontal(|ui| {
                            for asset in &mut *chunk {
                                ui.vertical(|ui| {
                                    ui.group(|ui| {
                                        ui.set_width(90.0);
                                        ui.set_height(110.0);
                                        ui.vertical_centered(|ui| {
                                            if asset.is_dir {
                                                ui.add_space(10.0);
                                                ui.label(egui::RichText::new("📁").size(28.0));
                                                ui.add_space(5.0);
                                            } else if asset.name.to_lowercase().ends_with(".png") {
                                                let texture = texture_cache.entry(asset.path.clone()).or_insert_with(|| {
                                                    load_texture_from_file(ctx, &asset.path)
                                                        .unwrap_or_else(|| {
                                                            ctx.load_texture(
                                                                &asset.path,
                                                                egui::ColorImage::example(),
                                                                egui::TextureOptions::default(),
                                                            )
                                                        })
                                                });

                                                ui.add(egui::Image::new((texture.id(), egui::vec2(45.0, 45.0))));

                                                if !asset.sub_sprites.is_empty() {
                                                    ui.horizontal(|ui| {
                                                        let arrow = if asset.is_expanded { "▼" } else { "▶" };
                                                        if ui.small_button(arrow).clicked() {
                                                            asset.is_expanded = !asset.is_expanded;
                                                        }
                                                    });
                                                }
                                            } else {
                                                ui.add_space(15.0);
                                                ui.label(egui::RichText::new("📄").size(32.0));
                                                ui.add_space(10.0);
                                            }

                                            let display_name = if asset.name.chars().count() > 10 {
                                                format!("{}...", asset.name.chars().take(8).collect::<String>())
                                            } else {
                                                asset.name.clone()
                                            };

                                            if ui.link(display_name).clicked() {
                                                if asset.is_dir {
                                                    *current_path = asset.path.clone();
                                                } else {
                                                    *selected_asset = Some(asset.clone());
                                                    *selected_object = None;
                                                }
                                            }
                                        });
                                    });
                                });
                                ui.add_space(4.0);
                            }
                        });

                        // Tiroir de sous-sprites
                        for asset in chunk {
                            if asset.is_expanded && !asset.sub_sprites.is_empty() {
                                ui.add_space(4.0);
                                egui::Frame::group(ui.style())
                                    .fill(Color32::from_rgb(25, 25, 25))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                egui::RichText::new(format!("Sub-sprites of : {}", asset.name))
                                                    .size(11.0)
                                                    .color(Color32::LIGHT_BLUE)
                                                    .strong()
                                            );
                                        });
                                        ui.add_space(4.0);

                                        egui::ScrollArea::horizontal()
                                            .id_source(format!("sub_scroll_{}", asset.path))
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    for sub in &asset.sub_sprites {
                                                        ui.vertical_centered(|ui| {
                                                            ui.group(|ui| {
                                                                ui.set_max_size(egui::vec2(65.0, 80.0));
                                                                ui.vertical_centered(|ui| {
                                                                    if let Some(texture) = texture_cache.get(&sub.path) {
                                                                        let mut img = egui::Image::new((texture.id(), egui::vec2(32.0, 32.0)));
                                                                        if let Some(uv) = sub.uv_rect {
                                                                            img = img.uv(Rect::from_min_max(
                                                                                egui::pos2(uv[0], uv[1]),
                                                                                egui::pos2(uv[2], uv[3]),
                                                                            ));
                                                                        }
                                                                        ui.add(img);
                                                                    }
                                                                    let sub_short = if sub.name.chars().count() > 8 {
                                                                        format!("{}..", sub.name.chars().take(6).collect::<String>())
                                                                    } else {
                                                                        sub.name.clone()
                                                                    };
                                                                    let response = ui.add(egui::Button::new(sub_short).sense(egui::Sense::click_and_drag()));
                                                                    if response.clicked(){
                                                                        *selected_asset = Some(sub.clone());
                                                                    }
                                                                    //Si utilisateur glisse l'element hors de sa case
                                                                    if response.dragged(){
                                                                        ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
                                                                        ui.ctx().data_mut(|d|{
                                                                            d.insert_temp(egui::Id::new("dragged_sprite"), sub.clone());
                                                                        });
                                                                    }
                                                                });
                                                            });
                                                        });
                                                        ui.add_space(6.0);
                                                    }
                                                });
                                            });
                                    });
                                ui.add_space(6.0);
                            }
                        }
                        ui.add_space(8.0);
                    }
                });
        });
}