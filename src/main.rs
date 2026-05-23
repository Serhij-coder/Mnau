use std::process::exit;

use macroquad::{miniquad::window::set_window_size, prelude::*};

use crate::{car::Car, cat::Cat, fish::Fish, textures::Assets, utils::GameOverAction};

mod car;
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
    let mut fish_spawn_timer: f32 = 0.0;

    let mut cars: Vec<Car> = Vec::new();
    let mut car_spawn_timer: f32 = 0.0;

    let mut elapsed: f32 = 0.0;

    let mut cat = Cat::new();
    let mut score: i64 = 0;

    loop {
        // 1. Logic (Update your variables here)
        let dt = get_frame_time();
        elapsed += dt;

        let fish_interval = (1.5 + 3.5 * (-elapsed / 50.0).exp()).max(1.5);
        let car_interval = (3.0 + 7.0 * (-elapsed / 55.0).exp()).max(3.0);
        let car_speed_min = (600.0 - 300.0 * (-elapsed / 60.0).exp()).min(600.0);
        let car_speed_max = (800.0 - 300.0 * (-elapsed / 60.0).exp()).min(800.0);

        fish_spawn_timer += dt;
        if fish_spawn_timer >= fish_interval {
            fishes.push(Fish::new());
            fish_spawn_timer = 0.0;
        }

        car_spawn_timer += dt;
        if car_spawn_timer >= car_interval {
            cars.push(Car::new(car_speed_min, car_speed_max));
            car_spawn_timer = 0.0;
        }

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

        // Check for car collisions
        let mut game_over = false;
        for car in &cars {
            if car.check_collision(cat.position) {
                println!("Game Over! Car collision detected!");
                game_over = true;
                break;
            }
        }

        if game_over {
            // Show game over screen and wait for player action
            loop {
                let action = utils::game_over_screen(&assets.font, score).await;
                match action {
                    GameOverAction::Retry => {
                        // Reset game state and restart
                        fishes.clear();
                        fish_spawn_timer = 0.0;
                        cars.clear();
                        car_spawn_timer = 0.0;
                        cat = Cat::new();
                        score = 0;
                        elapsed = 0.0;
                        break;
                    }
                    GameOverAction::Quit => {
                        exit(0);
                    }
                    GameOverAction::None => {
                        // Continue waiting for input
                    }
                }
            }
        }

        // Remove off-screen cars
        cars.retain(|car| !car.is_off_screen());

        // 2. Rendering (Clear first, then draw)
        clear_background(GRAY);

        for fish in &fishes {
            fish.update(&assets);
        }

        for car in &mut cars {
            car.update(&assets);
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
