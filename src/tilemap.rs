use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TileData {
    pub asset_path: String, //Chemin de la tuile sprite
    pub uv_rect: Option<[f32; 4]>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct  TileMap{
    pub tile_size: f32, //8.0 - 16.0 - 32.0 - 64.0 etc...
    //Structure x - y en grille clé et donnée de la tuile en valeur
    pub tiles: std::collections::HashMap<(i32, i32), TileData>,
}