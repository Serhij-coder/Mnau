use macroquad::{
    color::WHITE,
    input::{KeyCode, is_key_down},
    math::Vec2,
    texture::draw_texture,
    window::{screen_height, screen_width},
};

use crate::textures::Assets;

pub enum CatType {
    MouthOpen,
    MouthClose,
}

pub struct Cat {
    pub position: Vec2,
    pub variant: CatType,
    pub velocity: f32,
}

impl Cat {
    pub fn new() -> Self {
        let variant = CatType::MouthOpen;
        let position = Vec2::new(screen_width() / 2.0 - 50.0, screen_height() / 2.0 - 50.0);
        let velocity = 5.0;

        Cat {
            position,
            variant,
            velocity,
        }
    }

    pub fn update(&mut self, assets: &Assets) {
        self.check_cat_movement();
        self.draw(assets);
    }

    fn check_cat_movement(&mut self) {
        if is_key_down(KeyCode::D) {
            self.position.x += self.velocity;
            println!("Key pressed D");
        } else if is_key_down(KeyCode::A) {
            self.position.x -= self.velocity;
            println!("Key pressed A");
        }
        if is_key_down(KeyCode::S) {
            self.position.y += self.velocity;
            println!("Key pressed S");
        } else if is_key_down(KeyCode::W) {
            self.position.y -= self.velocity;
            println!("Key pressed W");
        }
    }

    fn draw(&self, assets: &Assets) {
        let texture = match self.variant {
            CatType::MouthOpen => &assets.cat,
            CatType::MouthClose => &assets.cat2,
        };

        draw_texture(texture, self.position.x, self.position.y, WHITE);
    }
}
