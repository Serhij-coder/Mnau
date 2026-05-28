use std::process::exit;

use macroquad::{miniquad::window::set_window_size, prelude::*};
use macroquad::rand::srand;

use crate::{car::Car, cat::Cat, fish::Fish, textures::Assets, utils::GameOverAction};

mod car;
mod cat;
mod fish;
mod textures;
mod utils;

#[macroquad::main("Mnau")]
async fn main() {
    set_fullscreen(true);

    srand(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64,
    );

    let font =
        load_ttf_font_from_bytes(include_bytes!("../res/font.otf")).expect("Failed to load font");

    utils::load_screen(&font).await;

    let assets = Assets::load(font).expect("Failed to load assets");

    let mut fishes: Vec<Fish> = Vec::new();
    let mut fish_spawn_timer: f32 = 0.0;

    let mut cars: Vec<Car> = Vec::new();
    let mut car_spawn_timer: f32 = 0.0;

    let mut elapsed: f32 = 0.0;

    let mut cat = Cat::new();
    let mut score: i64 = 0;
    let mut debug_visible: bool = false;

    let star_count = 150;
    let mut stars: Vec<(f32, f32, f32)> = (0..star_count)
        .map(|_| {
            let x = rand::gen_range(0.0, screen_width());
            let y = rand::gen_range(0.0, screen_height());
            let brightness = rand::gen_range(0.3, 1.0);
            (x, y, brightness)
        })
        .collect();

    let fish_interval_floor = 1.5;
    let fish_interval_amplitude = 1.0;
    let fish_interval_tau = 50.0;

    let car_interval_floor = 3.0;
    let car_interval_amplitude = 2.0;
    let car_interval_tau = 55.0;

    let car_speed_cap_min = 600.0;
    let car_speed_cap_max = 800.0;
    let car_speed_amplitude = 150.0;
    let car_speed_tau = 60.0;

    let car_double_threshold = 30.0;

    loop {
        // 1. Logic (Update your variables here)
        let dt = get_frame_time();
        elapsed += dt;

        let fish_interval = (fish_interval_floor
            + fish_interval_amplitude * (-elapsed / fish_interval_tau).exp())
        .max(fish_interval_floor);
        let car_interval = (car_interval_floor
            + car_interval_amplitude * (-elapsed / car_interval_tau).exp())
        .max(car_interval_floor);
        let car_speed_min = (car_speed_cap_min
            - car_speed_amplitude * (-elapsed / car_speed_tau).exp())
        .min(car_speed_cap_min);
        let car_speed_max = (car_speed_cap_max
            - car_speed_amplitude * (-elapsed / car_speed_tau).exp())
        .min(car_speed_cap_max);

        fish_spawn_timer += dt;
        if fish_spawn_timer >= fish_interval {
            fishes.push(Fish::new());
            fish_spawn_timer = 0.0;
        }

        car_spawn_timer += dt;
        if car_spawn_timer >= car_interval {
            cars.push(Car::new(car_speed_min, car_speed_max));
            if elapsed >= car_double_threshold {
                cars.push(Car::new(car_speed_min, car_speed_max));
            }
            car_spawn_timer = 0.0;
        }

        if is_key_pressed(KeyCode::Q) {
            exit(0);
        }

        if is_key_pressed(KeyCode::F3) {
            debug_visible = !debug_visible;
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
                        stars = (0..star_count)
                            .map(|_| {
                                let x = rand::gen_range(0.0, screen_width());
                                let y = rand::gen_range(0.0, screen_height());
                                let brightness = rand::gen_range(0.3, 1.0);
                                (x, y, brightness)
                            })
                            .collect();
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
        clear_background(Color::new(0.03, 0.03, 0.06, 1.0));

        for &(x, y, brightness) in &stars {
            draw_circle(
                x,
                y,
                1.5,
                Color::new(brightness, brightness, brightness, 1.0),
            );
        }

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

        // Draw game timer in the top-right corner
        let time_text = format!("{}s", elapsed as u64);
        let time_dims = measure_text(&time_text, Some(&assets.font), 50, 1.0);
        draw_text_ex(
            &time_text,
            screen_width() - time_dims.width - 10.0,
            60.0,
            TextParams {
                font: Some(&assets.font),
                font_size: 50,
                color: WHITE,
                ..Default::default()
            },
        );

        if debug_visible {
            let debug_x = screen_width() - 380.0;
            let debug_y = 20.0;
            let font_size = 24;
            let line_h = 26.0;
            let pad_y = 8.0;

            draw_rectangle(
                debug_x - 10.0,
                debug_y - pad_y,
                370.0,
                line_h * 9.0 + pad_y * 2.0,
                Color::new(0.0, 0.0, 0.0, 0.7),
            );

            let lines = [
                format!("FPS: {}", get_fps()),
                format!("Time: {:.1}s", elapsed),
                format!("Fish: every {:.2}s ({} alive)", fish_interval, fishes.len()),
                format!("Cars: every {:.2}s ({} alive)", car_interval, cars.len()),
                format!("Car speed: {:.0}-{:.0}", car_speed_min, car_speed_max),
                format!("Cat: ({:.0}, {:.0})", cat.position.x, cat.position.y),
                format!("Score: {}", score),
                format!("Double car: {}", if elapsed >= car_double_threshold { "ON" } else { "OFF" }),
                format!("[F3] hide debug"),
            ];
            for (i, line) in lines.iter().enumerate() {
                draw_text_ex(
                    line,
                    debug_x,
                    debug_y + i as f32 * line_h + pad_y,
                    TextParams {
                        font_size,
                        color: WHITE,
                        ..Default::default()
                    },
                );
            }
        }

        // 3. Wait for the next frame (Crucial!)
        next_frame().await
    }
}
