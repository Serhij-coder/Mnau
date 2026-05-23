use macroquad::prelude::*;

pub struct Assets {
    pub car_white: Texture2D,
    pub car_yellow: Texture2D,
    pub cat: Texture2D,
    pub cat2: Texture2D,
    pub fish_green: Texture2D,
    pub fish_skeleton: Texture2D,
    pub fish_white: Texture2D,
    pub font: Font,
}

fn load_embedded_texture(bytes: &[u8]) -> Texture2D {
    Texture2D::from_file_with_format(bytes, None)
}

impl Assets {
    pub fn load(font: Font) -> Result<Self, String> {
        let car_white = load_embedded_texture(include_bytes!("../res/car_white.png"));
        let car_yellow = load_embedded_texture(include_bytes!("../res/car_yellow.png"));
        let cat = load_embedded_texture(include_bytes!("../res/cat.png"));
        let cat2 = load_embedded_texture(include_bytes!("../res/cat2.png"));
        let fish_green = load_embedded_texture(include_bytes!("../res/fish_green.png"));
        let fish_skeleton = load_embedded_texture(include_bytes!("../res/fish_skeleton.png"));
        let fish_white = load_embedded_texture(include_bytes!("../res/fish_white.png"));

        Ok(Self {
            car_white,
            car_yellow,
            cat,
            cat2,
            fish_green,
            fish_skeleton,
            fish_white,
            font,
        })
    }
}
