use egui::{Color32, Pos2, Rect, TextureHandle, Vec2};
use crate::views::asset_browser::{AssetEntry, SpriteMeta};
pub struct SpriteEditorState {
    pub cols: usize,
    pub rows: usize,
    pub is_open: bool,
}

impl Default for SpriteEditorState {
    fn default() -> Self {
        Self{
            cols: 4,
            rows: 2,
            is_open: false,
        }
    }
}

pub fn render(
    ctx: &egui::Context,
    state: &mut SpriteEditorState,
    texture: Option<&TextureHandle>,
    selected_asset: &mut Option<AssetEntry>,
){
    if !state.is_open {return;}
    egui_window(ctx, state, texture, selected_asset);
}

fn egui_window(ctx: &egui::Context, state: &mut SpriteEditorState, texture: Option<&TextureHandle>, selected_asset: &mut Option<AssetEntry>) {
    let mut open = state.is_open;
    let mut apply_click = false;

    egui::Window::new("✂ Sprite Editor (Slice)")
        .open(&mut open)
        .default_size(Vec2::new(600.0, 500.0))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Grid Slicing Settings");
                ui.separator();
                ui.label("Columns:");
                ui.add(egui::DragValue::new(&mut state.cols).clamp_range(1.0..=8.0));
                ui.label("Rows:");
                ui.add(egui::DragValue::new(&mut state.rows).clamp_range(1.0..=8.0));
            });
            ui.separator();

            ui.allocate_ui_with_layout(
                ui.available_size(),
                egui::Layout::top_down(egui::Align::Center),
                |ui|{
                    if let Some(tex) = texture {
                        let available_size = ui.available_size() - Vec2::new(0.0, 64.0);
                        let tex_size = tex.size_vec2();
                        let scale = (available_size.x / tex_size.x).min(available_size.y / tex_size.y);
                        let display_size = tex_size * scale;

                        let (rect, _response) = ui.allocate_exact_size(display_size, egui::Sense::hover());

                        ui.painter().image(
                            tex.id(),
                            rect,
                            Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                            Color32::WHITE,
                        );

                        let painter = ui.painter();
                        let cell_w = rect.width() / state.cols as f32;
                        let cell_h = rect.height() / state.rows as f32;

                        for c in 0..state.cols {
                            let x = rect.min.x + c as f32 * cell_w;
                            painter.line_segment(
                                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                                egui::Stroke::new(1.0_f32, Color32::from_rgb(0, 255, 128)),
                            );
                        }

                        for r in 0..state.rows {
                            let y = rect.min.y + r as f32 * cell_h;
                            painter.line_segment(
                                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                                egui::Stroke::new(1.0_f32, Color32::from_rgb(0, 255, 128)),
                            );
                        }

                        ui.add_space(10.0);
                        ui.label(format!("Active Cut : {} with sub-sprite {}x{} grid", state.cols * state.rows, state.cols, state.rows));

                        if ui.button("✔ Slice & Apply").clicked() {
                            apply_click = true;
                        }
                    } else {
                        ui.colored_label(Color32::GRAY, "No textures selected for this sprite !");
                    }
                }
            );
        });

    // Synchronisation de l'état d'ouverture
    state.is_open = open;

    // Validation et écriture de la persistance
    if apply_click {
        if let Some(asset) = selected_asset {
            // 1. Sauvegarde propre du fichier .meta sans espace parasite dans le nom
            let meta_path_format = format!("{}.meta", asset.path);
            let meta_data = SpriteMeta {
                cols: state.cols,
                rows: state.rows,
            };

            if let Ok(json) = serde_json::to_string(&meta_data) {
                let _ = std::fs::write(meta_path_format, json);
            }

            // 2. Génération des sous-sprites en mémoire sur l'asset sélectionné
            asset.sub_sprites.clear();
            let total_cuts = state.cols * state.rows; // <-- Multiplication corrigée ici

            for i in 0..total_cuts {
                asset.sub_sprites.push(AssetEntry {
                    name: format!("{}_sub_{}", asset.name, i),
                    path: asset.path.clone(),
                    is_dir: false,
                    is_expanded: false,
                    sub_sprites: vec![],
                });
            }
            state.is_open = false;
        }
    }
}