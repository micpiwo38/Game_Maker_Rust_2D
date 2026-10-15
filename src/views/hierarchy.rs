use crate::views::scene::SceneEntity;

//Le panneau hierarchy
pub fn render(ctx: &egui::Context, selected_object: &mut Option<String>, scene_entities: &mut Vec<SceneEntity>) {
    egui::SidePanel::left("hierarchy_panel")
        .default_width(200.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Hierarchy");
            ui.separator();

            //Bouton rapide ajouter un objet
            if ui.button("+ Add Entity").clicked(){
                //Logique d'ajout d'un objet
            }
            ui.add_space(5.0);
            //Lister les objets de la scene
            let scroll_area = egui::ScrollArea::vertical().auto_shrink([false; 2]);
            scroll_area.show(ui, |ui| {
                for entity in scene_entities{
                    let is_selected = selected_object.as_ref() == Some(&entity.name);
                    //On affiche un bouton ou un label cliquable pour chaque objet
                    let response = ui.selectable_label(is_selected, &entity.name);
                    if response.clicked() {
                        //Au clic, on selectionne l'objet (Inspector le detecte et affiche les propeités)
                        *selected_object = Some(entity.name.clone())
                    }
                }
            });
        });
}