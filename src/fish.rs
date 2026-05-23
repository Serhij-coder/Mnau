use macroquad::{
    color::WHITE,
    math::Vec2,
    prelude::rand,
    rand::rand,
    texture::{Texture2D, draw_texture},
    window::{screen_height, screen_width},
};

use crate::textures::Assets;

pub enum FishType {
    Green,
    White,
    Skeleton,
}

impl FishType {
    pub fn base_points(&self) -> i64 {
        match self {
            FishType::Green => 300,
            FishType::White => 100,
            FishType::Skeleton => -200,
        }
    }

    fn to_string(&self) -> String {
        match self {
            FishType::Green => "Fish_Green".to_string(),
            FishType::White => "Fish_White".to_string(),
            FishType::Skeleton => "Fish_Skeleton".to_string(),
        }
    }
}

pub struct Fish {
    pub position: Vec2,
    pub variant: FishType,
}

impl Fish {
    pub fn new() -> Self {
        let variant = match rand::gen_range(0, 3) {
            0 => FishType::Green,
            1 => FishType::White,
            2 => FishType::Skeleton,
            _ => unreachable!(),
        };

        let position = Vec2::new(
            rand::gen_range(100.0, screen_width() - 200.0),
            rand::gen_range(100.0, screen_height() - 200.0),
        );

        println!("Fish created: {} {}", position, variant.to_string());

        Fish { position, variant }
    }

    pub fn update(&self, assets: &Assets) {
        self.draw(assets);
    }

    /// Check if the cat collides with this fish (both are 100x100)
    /// Returns true if collision detected
    pub fn check_collision(&self, cat_pos: Vec2) -> bool {
        const CAT_SIZE: f32 = 100.0;
        const FISH_SIZE: f32 = 100.0;
        
        // Simple AABB collision detection
        let cat_right = cat_pos.x + CAT_SIZE;
        let cat_bottom = cat_pos.y + CAT_SIZE;
        let fish_right = self.position.x + FISH_SIZE;
        let fish_bottom = self.position.y + FISH_SIZE;
        
        cat_pos.x < fish_right
            && cat_right > self.position.x
            && cat_pos.y < fish_bottom
            && cat_bottom > self.position.y
    }

    fn draw(&self, assets: &Assets) {
        let texture = match self.variant {
            FishType::Green => &assets.fish_green,
            FishType::White => &assets.fish_white,
            FishType::Skeleton => &assets.fish_skeleton,
        };

        draw_texture(texture, self.position.x, self.position.y, WHITE);
    }
}
