use macroquad::{
    color::WHITE,
    input::{KeyCode, is_key_down},
    math::Vec2,
    texture::draw_texture,
    time::get_frame_time,
    window::{screen_height, screen_width},
};

use crate::textures::Assets;

pub enum CatType {
    MouthOpen,
    MouthClose,
}

pub enum CatEdge {
    TopLeft(Vec2),
    TopRight(Vec2),
    BotRight(Vec2),
    BotLeft(Vec2),
}

pub struct Cat {
    pub position: Vec2,
    pub variant: CatType,
    pub timer: f32,
    pub velocity: f32,
}

impl Cat {
    pub fn new() -> Self {
        let variant = CatType::MouthOpen;
        let position = Vec2::new(screen_width() / 2.0 - 50.0, screen_height() / 2.0 - 50.0);
        let velocity = 500.0;
        let timer = 0.0;

        Cat {
            position,
            variant,
            velocity,
            timer,
        }
    }

    pub fn update(&mut self, assets: &Assets) {
        self.check_cat_movement();
        self.check_cat_variant();
        self.draw(assets);
    }

    fn draw(&self, assets: &Assets) {
        let texture = match self.variant {
            CatType::MouthOpen => &assets.cat,
            CatType::MouthClose => &assets.cat2,
        };

        draw_texture(texture, self.position.x, self.position.y, WHITE);
    }

    fn check_cat_variant(&mut self) {
        self.timer += get_frame_time();

        if self.timer >= 0.5 {
            // Toggle the variant
            self.variant = match self.variant {
                CatType::MouthOpen => CatType::MouthClose,
                CatType::MouthClose => CatType::MouthOpen,
            };

            // Reset timer to 0, or subtract 1.0 to keep it precise
            self.timer = 0.0;
        }
    }
}

// Movement
impl Cat {
    fn check_cat_movement(&mut self) {
        use CatEdge::*;

        // --- Horizontal Movement ---
        if is_key_down(KeyCode::D) {
            // Check if the NEXT step would put the Right edge off-screen
            if self.get_edge(TopRight(Vec2::ZERO)).x + self.velocity * get_frame_time()
                > screen_width()
            {
                self.set_by_edge(TopRight(Vec2::new(screen_width(), self.position.y)));
            } else {
                self.position.x += self.velocity * get_frame_time();
            }
        } else if is_key_down(KeyCode::A) {
            // Check if the NEXT step would put the Left edge off-screen (< 0)
            if self.get_edge(TopLeft(Vec2::ZERO)).x - self.velocity * get_frame_time() < 0.0 {
                self.set_by_edge(TopLeft(Vec2::new(0.0, self.position.y)));
            } else {
                self.position.x -= self.velocity * get_frame_time();
            }
        }

        // --- Vertical Movement ---
        if is_key_down(KeyCode::S) {
            // Check if the NEXT step would put the Bottom edge off-screen
            if self.get_edge(BotLeft(Vec2::ZERO)).y + self.velocity * get_frame_time()
                > screen_height()
            {
                self.set_by_edge(BotLeft(Vec2::new(self.position.x, screen_height())));
            } else {
                self.position.y += self.velocity * get_frame_time();
            }
        } else if is_key_down(KeyCode::W) {
            // Check if the NEXT step would put the Top edge off-screen (< 0)
            if self.get_edge(TopLeft(Vec2::ZERO)).y - self.velocity * get_frame_time() < 0.0 {
                self.set_by_edge(TopLeft(Vec2::new(self.position.x, 0.0)));
            } else {
                self.position.y -= self.velocity * get_frame_time();
            }
        }
    }
}

impl Cat {
    pub fn get_edge(&self, which: CatEdge) -> Vec2 {
        use CatEdge::*;
        let x = self.position.x;
        let y = self.position.y;

        match which {
            TopLeft(_) => Vec2::new(x, y),
            TopRight(_) => Vec2::new(x + 100.0, y),
            BotRight(_) => Vec2::new(x + 100.0, y + 100.0),
            BotLeft(_) => Vec2::new(x, y + 100.0),
        }
    }

    /// Set cat position based on provided edge
    pub fn set_by_edge(&mut self, edge: CatEdge) {
        use CatEdge::*;
        let mut set_edge = |v: Vec2| {
            self.position.x = v.x;
            self.position.y = v.y;
        };

        match edge {
            TopLeft(pos) => set_edge(Vec2::new(pos.x, pos.y)),
            TopRight(pos) => set_edge(Vec2::new(pos.x - 100.0, pos.y)),
            BotRight(pos) => set_edge(Vec2::new(pos.x - 100.0, pos.y - 100.0)),
            BotLeft(pos) => set_edge(Vec2::new(pos.x, pos.y - 100.0)),
        }
    }
}
