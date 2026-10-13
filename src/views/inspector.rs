
pub fn render(ctx: &egui::Context, selected_object: &mut Option<String>) {
    egui::SidePanel::right("inspector_panel")
        .default_width(250.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Inspector");
            ui.separator();
            if let Some(obj_name) = selected_object {
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(obj_name);
                });
                ui.add_space(10.0);
                ui.collapsing("Transform", |ui|{
                    //Valeur fictive de position exemple
                    let mut pos_x = 0.0f32;
                    let mut pos_y = 0.0f32;
                    ui.horizontal(|ui| {
                        ui.label("Position X : ");
                        ui.add(egui::DragValue::new(&mut pos_x).speed(1.0));
                    });
                    //Y
                    ui.horizontal(|ui| {
                        ui.label("Position Y : ");
                        ui.add(egui::DragValue::new(&mut pos_y).speed(1.0));
                    });
                });
            }else{
                ui.colored_label(
                    egui::Color32::GRAY,
                    "No object selected in the Hierarchy.",
                );
            }
        });
}