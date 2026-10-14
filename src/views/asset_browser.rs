use std::default::Default;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use egui::{Color32, TextureHandle};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AssetEntry{
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub sub_sprites: Vec<AssetEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SpriteMeta{
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
                                let total_cuts = meta.cols * meta.rows;
                                for i in 0..total_cuts {
                                    sub_sprites.push(AssetEntry {
                                        name: format!("{}_sub_{}", file_name, i),
                                        path: file_path.clone(),
                                        is_dir: false,
                                        is_expanded: false,
                                        sub_sprites: vec![],
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
                    });
                }
            }
        }
    }
    entries
}

pub fn load_texture_from_file(ctx: &egui::Context, path: &str) -> Option<TextureHandle>{
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
    assets: &mut [AssetEntry], // <-- IMPORTANT : Passé en mutable (&mut [AssetEntry]) pour modifier l'état d'expansion
    texture_cache: &mut HashMap<String, TextureHandle>,
    selected_asset: &mut Option<AssetEntry>,
    selected_object: &mut Option<String>
) {
    egui::TopBottomPanel::bottom("asset_browser_panel")
        .default_height(200.0)
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

                    // On parcourt les assets par chunks pour faire la grille principale
                    for chunk in assets.chunks_mut(columns) {
                        ui.horizontal(|ui| {
                            for asset in chunk {
                                ui.vertical(|ui| {
                                    // 1. La tuile principale du sprite
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

                                                // --- BOUTON FLÈCHE FONCTIONNEL ---
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

                                    // 2. --- AFFICHAGE DES SOUS-SPRITES DIRECTEMENT SOUS LA TUILE QUAND DÉPLIÉ ---
                                    if asset.is_expanded && !asset.sub_sprites.is_empty() {
                                        ui.add_space(4.0);
                                        ui.group(|ui| {
                                            ui.set_width(90.0);
                                            ui.vertical(|ui| {
                                                ui.label(egui::RichText::new("Sub-sprites:").size(10.0).color(Color32::YELLOW));
                                                egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                                                    for sub in &asset.sub_sprites {
                                                        ui.horizontal(|ui| {
                                                            if let Some(texture) = texture_cache.get(&sub.path) {
                                                                ui.add(egui::Image::new((texture.id(), egui::vec2(24.0, 24.0))));
                                                            }
                                                            if ui.link(&sub.name).clicked() {
                                                                *selected_asset = Some(sub.clone());
                                                            }
                                                        });
                                                    }
                                                });
                                            });
                                        });
                                    }
                                });
                                ui.add_space(4.0);
                            }
                        });
                        ui.add_space(6.0);
                    }
                });
        });
}