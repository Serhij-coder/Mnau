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
    fn base_points(&self) -> i64 {
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
            rand::gen_range(0.0, screen_width()),
            rand::gen_range(0.0, screen_height()),
        );

        println!("Fish created: {} {}", position, variant.to_string());

        Fish { position, variant }
    }

    pub fn update(&self, assets: &Assets) {
        self.draw(assets);
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
