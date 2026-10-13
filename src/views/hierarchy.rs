
pub fn render(ctx: &egui::Context, objects: &[&str], selected_object: &mut Option<String>) {
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
                for obj in objects {
                    let is_selected = selected_object.as_deref() == Some(*obj);
                    //Un label cliquable qui change d'apparence si il est selectionné
                    let response = ui.selectable_label(is_selected, *obj);
                    if response.clicked() {
                        *selected_object = Some(obj.to_string());
                    }
                }
            });
        });
}