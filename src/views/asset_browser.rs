use std::fs;
use std::path::Path;
use egui::Color32;

#[derive(Clone)]
pub struct AssetEntry{
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}
//Fonction pour scanner un dossier locale
pub fn load_directory(path: &str) -> Vec<AssetEntry>{
    let mut entries = Vec::new();

    if let Ok(read_dir) = fs::read_dir(path){
        for entry in read_dir.flatten(){
            if let Ok(file_type) = entry.file_type(){
                if let Some(file_name) = entry.file_name().to_str(){
                    let file_path = entry.path().to_string_lossy().into_owned();
                    entries.push(AssetEntry{
                        name: file_name.to_string(),
                        path: file_path,
                        is_dir: file_type.is_dir(),
                    });
                }
            }
        }
    }
    entries
}
pub fn render(ctx: &egui::Context, current_path: &mut String, assets: &[AssetEntry]){
    egui::TopBottomPanel::bottom("asset_browser_panel")
        .default_height(200.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Project / Assets");
                ui.separator();
                ui.label(format!("Path: {}", current_path));
                //Bouton pour remonté au dossier parent
                if current_path != "assets" && ui.button("⬆ Back").clicked(){
                    let path = Path::new(current_path);
                    if let Some(parent) = path.parent() {
                        *current_path = parent.to_string_lossy().into_owned();
                    }
                }
            });
            ui.separator();
            //Affichage dynamique des fichiers du dossier
            let _ = &egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    if assets.is_empty(){
                        ui.colored_label(Color32::LIGHT_RED, "Folder is empty !");
                    }
                    for asset in assets {
                        let icon = if asset.is_dir { "📁" } else { "📄" };
                        ui.group(|ui| {
                            ui.set_width(90.0);
                            ui.vertical_centered(|ui|{
                                ui.label(egui::RichText::new(icon).size(24.0));
                                if ui.link(&asset.name).clicked() {
                                    if asset.is_dir {
                                        //Logique de navigation dans les sous-dossier
                                        *current_path = asset.path.clone();
                                    }
                                }
                            });
                        });
                        ui.add_space(5.0);
                    }
                });
            });
        });
}