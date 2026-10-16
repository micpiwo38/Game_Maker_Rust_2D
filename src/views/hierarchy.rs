use rfd::MessageDialogResult::No;
use crate::views::scene::SceneEntity;

//Le panneau hierarchy
pub fn render(
    ctx: &egui::Context,
    selected_object: &mut Option<String>,
    scene_entities: &mut Vec<SceneEntity>,
    focus_target: &mut Option<String>,
) {
    egui::SidePanel::left("hierarchy_panel")
        .default_width(200.0)
        .max_width(300.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Hierarchy");
            ui.separator();

            //Bouton rapide ajouter un objet
            if ui.button("+ Add Entity").clicked(){
                //Logique d'ajout d'un objet
            }
            ui.add_space(5.0);
            //Variable temporaire pour stocker un element a supprimer
            let mut entity_to_delete: Option<String> = None;
            //Lister les objets de la scene
            let scroll_area = egui::ScrollArea::vertical().auto_shrink([false; 2]);
            scroll_area.show(ui, |ui| {
                for entity in &*scene_entities{
                    let is_selected = selected_object.as_ref() == Some(&entity.name);
                    //On affiche un bouton ou un label cliquable pour chaque objet
                    let response = ui.selectable_label(is_selected, &entity.name);
                    //Clic affiche les propriétées dans Inspector
                    if response.clicked() {
                        //Au clic, on selectionne l'objet (Inspector le detecte et affiche les propeités)
                        *selected_object = Some(entity.name.clone())
                    }
                    //Double clic => focus sur le Transform de l'object selectionné
                    if response.double_clicked() {
                        *selected_object = Some(entity.name.clone());
                        *focus_target = Some(entity.name.clone());
                    }
                }
            });
            if let Some(selected_name) = selected_object{
                if ctx.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)) {
                    entity_to_delete = Some(selected_name.clone());
                }
            }
            //Effectue la supression proprement apres affichage
            if let Some(name_to_del) = entity_to_delete {
                scene_entities.retain(|entity| entity.name != name_to_del);
                *selected_object = None;
            }
        });
}