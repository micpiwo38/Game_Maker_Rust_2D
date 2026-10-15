use crate::views::asset_browser::AssetEntry;
use crate::views::scene::SceneEntity;
use crate::views::sprite_editor::SpriteEditorState;

//Enum pour gerer le decoupage du spritesheet
#[derive(PartialEq, Debug)]
pub enum SpriteImportMode{
    Single, Multiple,
}
pub struct SpriteImportSettings{
    pub mode: SpriteImportMode,
    pub pixels_per_unit: f32
}
//Le rendu du panneau Inspector
pub fn render(
    ctx: &egui::Context,
    selected_object: &mut Option<String>,
    selected_asset: &mut Option<AssetEntry>,
    sprite_editor_state: &mut SpriteEditorState,
    scene_entities: &mut Vec<SceneEntity>,
) {
    egui::SidePanel::right("inspector_panel")
        .default_width(280.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Inspector");
            ui.separator();

            //1. Cas ou un objet est selectionné dans la scene (Hierarchie)
            if let Some(obj_name) = selected_object {
                ui.horizontal(|ui| {
                    ui.label("Object Name:");
                    ui.text_edit_singleline(obj_name); //Afficher le nom de l'objet
                    ui.strong(obj_name.clone()) //Mettre en gras pour mettre en valeur l'objet selectionné
                });
                //On cherche l'entité dans la liste de scene
                if let Some(entity) = scene_entities.iter_mut().find(|e| e.name == *obj_name){
                    //Paneau des composants (Transform, Rigidbody2D, BoxCollider2D, etc...)
                    egui::CollapsingHeader::new("📍 Transform")
                        .default_open(true) //Ouvert par defaut
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Position X : ");
                                ui.add(egui::DragValue::new(&mut entity.x ).speed(1.0)); //Changer la position X du GameObject
                            });
                            ui.horizontal(|ui| {
                                ui.label("Position Y : ");
                                ui.add(egui::DragValue::new(&mut entity.y).speed(1.0));
                            })
                        });
                }else{
                    //Pas de GameObject Trouvé
                    ui.colored_label(egui::Color32::LIGHT_RED, "No matching entity found in scene !");
                }
            }else if let Some(asset) = selected_asset{
                //2. Cas ou on selectionne un asset dans le panneau du bas Project / Asset
                ui.colored_label(egui::Color32::LIGHT_BLUE, "📁 Asset Importer Settings");
                ui.separator();
                ui.label(format!("File: {}", asset.name));
                ui.label(egui::RichText::new(&asset.path).size(10.0).color(egui::Color32::DARK_GREEN));
                ui.add_space(10.0);
                //Si c un fichier .png => on affiche les option d'importation du sprite
                if asset.name.to_lowercase().ends_with(".png"){
                    ui.collapsing("Sprite 2D Settings", |ui|{
                        ui.label("Texture Type: Sprite (2D and UI)");
                        //Choix du mode de découpe (Single ou Multiple)
                        //------------------SINGLE POUR L'INSTANT A MODIFIER------------------//
                        let mut mode = SpriteImportMode::Multiple;
                        ui.label("Sprite Mode:");
                        ui.horizontal(|ui|{
                            ui.radio_value(&mut mode, SpriteImportMode::Single, "Single");
                            ui.radio_value(&mut mode, SpriteImportMode::Multiple, "Multiple");
                        });
                        if mode == SpriteImportMode::Multiple{
                            ui.add_space(5.0);
                            if ui.button("✂ Open Sprite Editor (Slice)").clicked() {
                                // Ouvre une fenêtre de découpage du SpriteEditor.rs
                                sprite_editor_state.is_open = true;
                            }
                            ui.colored_label(egui::Color32::YELLOW, "Mode Multiple : Cut spritesheet with grid");
                        }else{
                            ui.colored_label(egui::Color32::GREEN, "Mode Single : Get single sprite");
                        }
                        ui.add_space(5.0);
                        let mut ppu = 16.0f32; //Pixel per unit = 16 px par defaut
                        ui.horizontal(|ui|{
                            ui.label("Pixels Per Unit: (PPU)");
                            ui.add(egui::DragValue::new(&mut ppu).speed(1.0).clamp_range(8.0..=128.0)); //Slider type de découpe de 8px a 128px
                        });
                    });
                    ui.add_space(15.0);
                    if ui.button("Apply Changes").clicked(){
                        println!("Apply Changes");
                    }
                }else{
                    ui.colored_label(egui::Color32::GRAY, "No import settings available for this file type !");
                }
            } else{
                //3. Cas ou rien n'est selectionné
                ui.colored_label(
                    egui::Color32::GRAY,
                    "No object or asset selected in scene!",
                );
            }
        });
}