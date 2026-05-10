use std::fmt::format;

use macroquad::texture::{Texture2D, load_texture};

pub async fn load_texture_util(path: &str) -> Texture2D {
    let error_message = format!("Failed to load texture: {}", path.to_string());
    load_texture(path).await.expect(&error_message)
}
