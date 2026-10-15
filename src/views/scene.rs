use std::fs;
use serde::{Deserialize, Serialize};
use crate::views::asset_browser::AssetEntry;

#[derive(Serialize, Deserialize, Clone)]
pub struct SceneEntity{
    pub name: String,
    pub asset_path: String,
    pub uv_rect: Option<[f32;4]>,
    pub x: f32, //Position X et Y dans Viewport
    pub y: f32
}

//Fonction pour sauver une scene
pub fn save_scene(scene_entities: &[SceneEntity]){
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("Game Scene", &["json"])
        .save_file(){
        if let Ok(json) = serde_json::to_string_pretty(scene_entities){
            let _ = fs::write(path, json);
        }
    }
}

//Fonction pour charger une scene => File -> Open
pub fn load_scene(scene_entities: &mut Vec<SceneEntity>){
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("Game Scene", &["json"])
        .pick_file(){
        if let Ok(content) = fs::read_to_string(path){
            if let Ok(loaded_entities) = serde_json::from_str::<Vec<SceneEntity>>(&content){
                *scene_entities = loaded_entities;
            }
        }
    }
}
