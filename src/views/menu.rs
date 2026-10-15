
use crate::views::scene::{self, SceneEntity};

pub fn render(ctx: &egui::Context, scene_entites: &mut Vec<SceneEntity>) {
    egui::TopBottomPanel::top("top_menu_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                //Creer une nouvelle scene
                if ui.button("New Scene").clicked() {
                    //Logique de la nouvelle scene
                    scene_entites.clear();
                    ui.close_menu();
                }
                //Sauvegarder la scene
                if ui.button("Save Scene").clicked() {
                    //Sauvegarde de la scene
                    scene::save_scene(scene_entites);
                    ui.close_menu();
                }
                //Charger une scene
                if ui.button("Load Scene").clicked() {
                    //Sauvegarde de la scene
                    scene::load_scene(scene_entites);
                    ui.close_menu();
                }
                //Quitter l'outil
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