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

    utils::load_screen(&font).await;

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
