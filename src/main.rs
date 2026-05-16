use std::process::exit;

use macroquad::{miniquad::window::set_window_size, prelude::*};

use crate::{cat::Cat, fish::Fish, textures::Assets};

mod cat;
mod fish;
mod textures;
mod utils;

#[macroquad::main("Mnau")]
async fn main() {
    set_fullscreen(true);

    let font =
        load_ttf_font_from_bytes(include_bytes!("../res/font.otf")).expect("Failed to load font");

    let mut loadbar_width = 0;

    for _ in 0..170 {
        clear_background(BLACK);

        let text = "MNAU!";
        let font_size = 60;

        // 1. Measure the text using your custom font
        let dimensions = measure_text(text, Some(&font), font_size, 1.0);

        // 2. Calculate the center of the screen
        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // 3. Draw using the calculated offsets
        draw_text_ex(
            text,
            center_x - (dimensions.width / 2.0),
            center_y + (dimensions.height / 2.0) - dimensions.offset_y,
            TextParams {
                font: Some(&font),
                font_size,
                color: WHITE,
                ..Default::default()
            },
        );

        draw_rectangle(
            screen_width() / 2.0 - dimensions.width,
            screen_height() / 2.0 + dimensions.height,
            loadbar_width as f32,
            2.0,
            WHITE,
        );

        loadbar_width += 2;

        next_frame().await;
    }

    let assets = match Assets::load(font).await {
        Ok(assets) => assets,
        Err(e) => panic!("{}", e),
    };

    let mut fishes: Vec<Fish> = Vec::new();

    for _ in 0..10 {
        fishes.push(Fish::new());
    }

    let mut cat = Cat::new();

    loop {
        // 1. Logic (Update your variables here)
        if is_key_pressed(KeyCode::Q) {
            exit(0);
        }

        // 2. Rendering (Clear first, then draw)
        clear_background(GRAY);

        for fish in &fishes {
            fish.update(&assets);
        }

        cat.update(&assets);

        // 3. Wait for the next frame (Crucial!)
        next_frame().await
    }
}
