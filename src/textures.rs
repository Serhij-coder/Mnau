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

impl Assets {
    pub async fn load(font: Font) -> Result<Self, String> {
        // We load them all concurrently
        let car_white = load_texture("res/car_white.png")
            .await
            .map_err(|_| "Failed to load car_white")?;
        let car_yellow = load_texture("res/car_yellow.png")
            .await
            .map_err(|_| "Failed to load car_yellow")?;
        let cat = load_texture("res/cat.png")
            .await
            .map_err(|_| "Failed to load cat")?;
        let cat2 = load_texture("res/cat2.png")
            .await
            .map_err(|_| "Failed to load cat2")?;
        let fish_green = load_texture("res/fish_green.png")
            .await
            .map_err(|_| "Failed to load fish_green")?;
        let fish_skeleton = load_texture("res/fish_skeleton.png")
            .await
            .map_err(|_| "Failed to load fish_skeleton")?;
        let fish_white = load_texture("res/fish_white.png")
            .await
            .map_err(|_| "Failed to load fish_white")?;

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
