use macroquad::{
    color::WHITE,
    math::Vec2,
    prelude::rand,
    texture::{draw_texture, Texture2D},
    window::{screen_height, screen_width},
};

#[derive(Clone, Copy)]
pub enum CarSide {
    Left,
    Right,
    Top,
    Bottom,
}

pub struct Car {
    pub position: Vec2,
    pub velocity: f32,
    pub direction: Vec2,
    pub is_white: bool,
}

impl Car {
    pub fn new(min_velocity: f32, max_velocity: f32) -> Self {
        let side = match rand::gen_range(0, 4) {
            0 => CarSide::Left,
            1 => CarSide::Right,
            2 => CarSide::Top,
            3 => CarSide::Bottom,
            _ => CarSide::Left,
        };

        let is_white = rand::gen_range(0, 2) == 0;

        let (position, direction) = match side {
            CarSide::Left => {
                let pos = Vec2::new(-100.0, rand::gen_range(100.0, screen_height() - 100.0));
                let dir = Vec2::new(1.0, 0.0);
                (pos, dir)
            }
            CarSide::Right => {
                let pos = Vec2::new(
                    screen_width() + 100.0,
                    rand::gen_range(100.0, screen_height() - 100.0),
                );
                let dir = Vec2::new(-1.0, 0.0);
                (pos, dir)
            }
            CarSide::Top => {
                let pos = Vec2::new(rand::gen_range(100.0, screen_width() - 100.0), -100.0);
                let dir = Vec2::new(0.0, 1.0);
                (pos, dir)
            }
            CarSide::Bottom => {
                let pos = Vec2::new(
                    rand::gen_range(100.0, screen_width() - 100.0),
                    screen_height() + 100.0,
                );
                let dir = Vec2::new(0.0, -1.0);
                (pos, dir)
            }
        };

        let velocity = rand::gen_range(min_velocity, max_velocity);

        Car {
            position,
            velocity,
            direction,
            is_white,
        }
    }

    pub fn update(&mut self, assets: &crate::textures::Assets) {
        self.position += self.direction * self.velocity * macroquad::time::get_frame_time();
        
        let texture = if self.is_white {
            &assets.car_white
        } else {
            &assets.car_yellow
        };
        draw_texture(texture, self.position.x, self.position.y, WHITE);
    }

    pub fn is_off_screen(&self) -> bool {
        self.position.x < -150.0
            || self.position.x > screen_width() + 150.0
            || self.position.y < -150.0
            || self.position.y > screen_height() + 150.0
    }

    pub fn check_collision(&self, cat_pos: Vec2) -> bool {
        const CAR_SIZE: f32 = 100.0;
        const CAT_SIZE: f32 = 100.0;

        let car_right = self.position.x + CAR_SIZE;
        let car_bottom = self.position.y + CAR_SIZE;
        let cat_right = cat_pos.x + CAT_SIZE;
        let cat_bottom = cat_pos.y + CAT_SIZE;

        self.position.x < cat_right
            && car_right > cat_pos.x
            && self.position.y < cat_bottom
            && car_bottom > cat_pos.y
    }
}