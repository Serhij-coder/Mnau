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
    let mut score: i64 = 0;

    loop {
        // 1. Logic (Update your variables here)
        if is_key_pressed(KeyCode::Q) {
            exit(0);
        }

        // Check for fish collisions and collect them
        let mut collected_indices = Vec::new();
        
        for (index, fish) in fishes.iter().enumerate() {
            if fish.check_collision(cat.position) {
                score += fish.variant.base_points();
                collected_indices.push(index);
                println!("Collected fish! Points: {}. Total score: {}", fish.variant.base_points(), score);
            }
        }

        // Remove collected fish (iterate in reverse to avoid index issues)
        for index in collected_indices.iter().rev() {
            fishes.remove(*index);
        }

        // 2. Rendering (Clear first, then draw)
        clear_background(GRAY);

        for fish in &fishes {
            fish.update(&assets);
        }

        cat.update(&assets);

        // Draw score on screen using custom font
        draw_text_ex(
            &format!("Score: {}", score),
            10.0,
            60.0,
            TextParams {
                font: Some(&assets.font),
                font_size: 50,
                color: WHITE,
                ..Default::default()
            },
        );

        // 3. Wait for the next frame (Crucial!)
        next_frame().await
    }
}
