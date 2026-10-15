use egui::{Color32, Sense};
use crate::views::asset_browser::AssetEntry;

pub fn render(
    ctx: &egui::Context,
    scene_objects: &mut Vec<String>, //Ajouter un objet dans la Hierarchie
    selected_objects: &mut Option<String>,
){
    egui::CentralPanel::default().show(ctx, |ui|{
        ui.horizontal(|ui|{
            ui.heading("Viewport");
            ui.add_space(20.0);
            if ui.button("▶ Play").clicked() {
                // Logique pour lancer le jeu plus tard
            }
            if ui.button("⏹ Stop").clicked() {
                // Logique pour stopper le jeu plus tard
            }
        });
        ui.separator();
        //Recuperer l'espace disponible au centre de l'ecran
        let available_size = ui.available_size();
        let (rect, _response) = ui.allocate_exact_size(available_size, Sense::hover());
        //On colore le fond en bleu-gris pour simuler la camera Unity
        ui.painter().rect_filled(
            rect,
            0.0,
            egui::Color32::from_rgb(15,18,25),
        );
        //Afficher un texte centré dans la zone de rendu
        ui.allocate_ui_at_rect(rect, |ui|{
            ui.centered_and_justified(|ui|{
                ui.colored_label(
                    Color32::from_gray(140),
                    "[ Game Viewport Area ]\n(Sprites and Tilemaps will render here !)",
                );
            });
        });
        //Gestion du DROP GameObject
        //Si on relache le bouton de la souris
        if ui.input(|i| i.pointer.any_released()){
            //Si on est dans la zone viewport
            if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()){
                if rect.contains(pointer_pos){
                    //On recupère le sprite qui est train d'etre glissé
                    if let Some(dragged_sprite) = ctx.data_mut(|d| d.remove_temp::<AssetEntry>(egui::Id::new("dragged_sprite"))){
                        //Spawn l'objet dans la scene
                        let new_object_name = format!("Sprite_{}", dragged_sprite.name);
                        scene_objects.push(new_object_name.clone());
                        *selected_objects = Some(new_object_name);
                    }
                }
            }
        }
    });
}