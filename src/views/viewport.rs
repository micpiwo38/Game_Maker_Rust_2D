use egui::{Color32, Rect, Sense, TextureHandle, Vec2};
use egui::ahash::HashMap;
use crate::views::asset_browser::AssetEntry;
use crate::views::scene::SceneEntity;

pub fn render(
    ctx: &egui::Context,
    scene_entities: &mut Vec<SceneEntity>, //Ajouter un objet dans la Hierarchie
    selected_objects: &mut Option<String>,
    texture_cache: &mut HashMap<String, TextureHandle>
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

        //Rendu des sprites DRAG - DROP dans le panneau Viewport
        for entity in scene_entities.iter(){
            //Si la texture existe => on la met en cache avec son chemin
            if let Some(texture) = texture_cache.get(&entity.asset_path){
                let sprite_size = Vec2::new(64.0,64.0);
                //Position absolue dans le restangle panneau viewport
                let abs_pos = rect.min + egui::vec2(entity.x, entity.y);
                let sprite_rect = Rect::from_min_size(abs_pos, sprite_size);

                let mut img = egui::Image::new((texture.id(), sprite_size));
                //Les position X et Y
                if let Some(uv) = entity.uv_rect{
                    img = img.uv(Rect::from_min_max(
                        egui::pos2(uv[0], uv[1]),
                        egui::pos2(uv[2], uv[3])
                    ));
                }
                //Afficher le sprite dans le viewport
                ui.put(sprite_rect, img);
                //Effet focus - selection
                if let Some(selected_name) = selected_objects{
                    if selected_name == &entity.name{
                        ui.painter().rect_stroke(
                            sprite_rect,
                            0.0,
                            egui::Stroke::new(2.0, Color32::from_rgb(0,200,255))
                        );
                    }
                }
            }
        }

        //Si la scene est vide => on affiche un message au centre
        if scene_entities.is_empty(){
            ui.allocate_ui_at_rect(rect, |ui|{
                ui.centered_and_justified(|ui|{
                    ui.colored_label(
                        Color32::from_gray(140),
                        "[ Game Viewport Area ]\n(Drag & Drop sprites here !)",
                    );
                });
            });
        }

        //Gestion du DROP GameObject
        //Si on relache le bouton de la souris
        if ui.input(|i| i.pointer.any_released()){
            //Si on est dans la zone viewport
            if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()){
                if rect.contains(pointer_pos){
                    //On recupère le sprite qui est train d'etre glissé
                    if let Some(dragged_sprite) = ctx.data_mut(|d| d.remove_temp::<AssetEntry>(egui::Id::new("dragged_sprite"))){
                        //Calcul de la position relative a l'interieur du panneau Viewport
                        let local_vec = pointer_pos - rect.min;
                        let local_pos = egui::pos2(local_vec.x, local_vec.y);
                        let new_entity = SceneEntity{
                            name: format!("Sprite_{}", dragged_sprite.name),
                            asset_path: dragged_sprite.path.clone(),
                            uv_rect: dragged_sprite.uv_rect,
                            x: local_pos.x,
                            y: local_pos.y,
                        };
                        scene_entities.push(new_entity);
                        *selected_objects = Some(format!("Sprite_{}", dragged_sprite.name));
                    }
                }
            }
        }
    });
}