use std::fs;
use serde::{Deserialize, Serialize};
use crate::views::asset_browser::AssetEntry;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SceneEntity{
    pub name: String,
    pub asset_path: String,
    pub uv_rect: Option<[f32;4]>,
    pub x: f32, //Position X et Y dans Viewport
    pub y: f32
}

//Fonction pour sauver une scene
pub fn save_scene(scene_entities: &[SceneEntity]){
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("Game Scene", &["json"])
        .save_file(){
        if let Ok(json) = serde_json::to_string_pretty(scene_entities){
            let _ = fs::write(path, json);
        }
    }
}

//Fonction pour charger une scene => File -> Open
pub fn load_scene(scene_entities: &mut Vec<SceneEntity>){
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("Game Scene", &["json"])
        .pick_file(){
        if let Ok(content) = fs::read_to_string(path){
            if let Ok(loaded_entities) = serde_json::from_str::<Vec<SceneEntity>>(&content){
                *scene_entities = loaded_entities;
            }
        }
    }
}

//Afficher un toast + message = fontion utilitaire generique
pub fn render_toast(
    ctx: &egui::Context,
    timer: &mut f32,
    message: &str,
    accent_color: egui::Color32
){
    if *timer > 0.0 {
        let alpha = (*timer / 1.0).clamp(0.0, 1.0);
        let screen_rect = ctx.screen_rect();
        // Positionnée en bas au centre de l'écran
        let window_pos = egui::pos2(screen_rect.center().x, screen_rect.max.y - 60.0);

        egui::Window::new("notification_toast")
            .fixed_pos(window_pos)
            .pivot(egui::Align2::CENTER_BOTTOM)
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .frame(
                egui::Frame::popup(&*ctx.style())
                    .fill(egui::Color32::from_black_alpha((200.0 * alpha) as u8)) // <-- Correction ici (*)
                    .rounding(8.0)
                    .inner_margin(12.0)
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(
                        accent_color.linear_multiply(alpha),
                        "✔"
                    );
                    ui.colored_label(
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * alpha) as u8), // <-- Correction ici (*)
                        message
                    );
                });
            });

        // Décrémente le timer à chaque frame
        *timer -= ctx.input(|i| i.stable_dt);
    }
}
