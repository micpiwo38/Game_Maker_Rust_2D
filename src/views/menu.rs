
pub fn render(ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_menu_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New Scene").clicked() {
                    //Logique de la nouvelle scene
                }
                if ui.button("Save Scene").clicked() {
                    //Sauvegarde de la scene
                }
                ui.separator();
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
            ui.menu_button("Edit", |_ui|{
                //Edition Undo, Redo, etc...
            });
            ui.menu_button("Window", |_ui|{
                //Gestion des panneau de l'editeur
            });
            ui.menu_button("Help", |ui| {
                //A propos, Docs, Version ...
            });
        });
    });
}