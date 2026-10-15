
#[derive(Clone)]
pub struct SceneEntity{
    pub name: String,
    pub asset_path: String,
    pub uv_rect: Option<[f32;4]>,
    pub position: egui::Pos2 //Position X et Y dans Viewport
}