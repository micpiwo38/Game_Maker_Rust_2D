use egui::{Color32, Sense};

pub fn render(ctx: &egui::Context){
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
        //Afficher un texte centré dans la zone de rendu
        ui.allocate_ui_at_rect(rect, |ui|{
            ui.centered_and_justified(|ui|{
                ui.colored_label(
                    Color32::from_gray(140),
                    "[ Game Viewport Area ]\n(Sprites and Tilemaps will render here !)",
                );
            });
        });
    });
}