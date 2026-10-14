use std::default::Default;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use egui::{Color32, TextureHandle};


#[derive(Clone)]
pub struct AssetEntry{
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub is_expanded: bool, //Affiche les sprites découpés ou non
    pub sub_sprites: Vec<AssetEntry>, //Les sous-sprites découpés
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
                        is_expanded: false,
                        sub_sprites: vec![],
                    });
                }
            }
        }
    }
    entries
}

//Charge des images (png, jpeg) et les transformes en textures
pub fn load_texture_from_file(ctx: &egui::Context, path: &str) -> Option<TextureHandle>{
    let image_bytes = fs::read(path).ok()?;
    let dynamic_image = image::load_from_memory(&image_bytes).ok()?;
    let rgba_image = dynamic_image.to_rgba8();
    let width = rgba_image.width() as usize;
    let height = rgba_image.height() as usize;
    let pixels = rgba_image.into_raw();
    //Securité si la taille du buffer ne corespond pas exactement au calcul => proprement au lieu de paniquer
    let egui_image = egui::ColorImage::from_rgba_unmultiplied([width, height], &pixels);
    //Supprimer le flou (Blur) des images
    Some(ctx.load_texture(path, egui_image, egui::TextureOptions::NEAREST))
}

pub fn render(
    ctx: &egui::Context,
    current_path: &mut String,
    assets: &[AssetEntry],
    texture_cache: &mut HashMap<String, TextureHandle>,
    selected_asset: &mut Option<AssetEntry>,
    selected_object: &mut Option<String>
) {
    egui::TopBottomPanel::bottom("asset_browser_panel")
        .default_height(200.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Project / Assets");
                ui.separator();
                ui.label(format!("Path: {}", current_path));

                if current_path != "assets" && ui.button("⬆ Back").clicked() {
                    let path = Path::new(current_path);
                    if let Some(parent) = path.parent() {
                        *current_path = parent.to_string_lossy().into_owned();
                    }
                }
            });
            ui.separator();

            // --- ZONE DE DÉFILEMENT VERTICAL AVEC GRILLE CALCULÉE ---
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if assets.is_empty() {
                        ui.colored_label(Color32::LIGHT_RED, "Folder is empty !");
                        return;
                    }

                    // 1. On calcule combien de colonnes tiennent dans la largeur du panneau
                    let available_width = ui.available_width();
                    let item_width = 100.0; // Largeur approximative d'une tuile (90px + marges)
                    let columns = (available_width / item_width).floor().max(1.0) as usize;

                    // 2. On découpe la liste des assets en paquets (lignes) de taille 'columns'
                    for chunk in assets.chunks(columns) {
                        ui.horizontal(|ui| {
                            for asset in chunk {
                                ui.group(|ui| {
                                    ui.set_width(90.0);
                                    ui.set_height(110.0);
                                    ui.vertical_centered(|ui| {
                                        if asset.is_dir {
                                            ui.add_space(10.0);
                                            ui.label(egui::RichText::new("📁").size(28.0));
                                            ui.add_space(5.0);
                                        } else if asset.name.to_lowercase().ends_with(".png") {
                                            let texture = texture_cache.entry(asset.path.clone()).or_insert_with(|| {
                                                load_texture_from_file(ctx, &asset.path)
                                                    .unwrap_or_else(|| {
                                                        ctx.load_texture(
                                                            &asset.path,
                                                            egui::ColorImage::example(),
                                                            egui::TextureOptions::default(),
                                                        )
                                                    })
                                            });

                                            ui.add(egui::Image::new((texture.id(), egui::vec2(45.0, 45.0))));
                                        } else {
                                            ui.add_space(10.0);
                                            ui.label(egui::RichText::new("📄").size(28.0));
                                            ui.add_space(5.0);
                                        }

                                        let display_name = if asset.name.chars().count() > 10 {
                                            format!("{}...", asset.name.chars().take(8).collect::<String>())
                                        } else {
                                            asset.name.clone()
                                        };

                                        if ui.link(display_name).clicked() {
                                            if asset.is_dir {
                                                *current_path = asset.path.clone();
                                            }else{
                                                //Clic sur un fichier => affiche dans le panneau Inspector
                                                *selected_asset = Some(asset.clone());
                                                *selected_object = None; //Deselectionne la hierarchie pour donner le focus a l'asset
                                            }
                                        }
                                    });
                                });
                                ui.add_space(4.0);
                            }
                        });
                        ui.add_space(4.0); // Espace vertical entre les lignes
                    }
                });
        });
}