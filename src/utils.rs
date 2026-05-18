use macroquad::{
    color::{BLACK, WHITE, RED, GREEN},
    input::{is_mouse_button_pressed, mouse_position, MouseButton},
    shapes::draw_rectangle,
    text::{Font, TextParams, draw_text_ex, measure_text},
    window::{clear_background, next_frame, screen_height, screen_width},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameOverAction {
    Retry,
    Quit,
    None,
}

pub async fn load_screen(font: &Font) {
    let mut loadbar_width = 0;

    for _ in 0..170 {
        clear_background(BLACK);

        let text = "MNAU!";
        let font_size = 60;

        // 1. Measure the text using your custom font
        let dimensions = measure_text(text, Some(&font), font_size, 1.0);

        // 2. Calculate the center of the screen
        let center_x = screen_width() / 2.0;
        let center_y = { screen_height() / 2.0 };

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
}

pub async fn game_over_screen(font: &Font, score: i64) -> GameOverAction {
    const BUTTON_WIDTH: f32 = 200.0;
    const BUTTON_HEIGHT: f32 = 60.0;
    const BUTTON_GAP: f32 = 40.0;

    loop {
        clear_background(BLACK);

        // Draw "GAME OVER" title
        let title_text = "GAME OVER";
        let title_font_size = 80;
        let title_dimensions = measure_text(title_text, Some(&font), title_font_size, 1.0);
        let center_x = screen_width() / 2.0;
        let title_y = screen_height() / 2.0 - 150.0;

        draw_text_ex(
            title_text,
            center_x - (title_dimensions.width / 2.0),
            title_y,
            TextParams {
                font: Some(&font),
                font_size: title_font_size,
                color: RED,
                ..Default::default()
            },
        );

        // Draw score
        let score_text = format!("Score: {}", score);
        let score_font_size = 60;
        let score_dimensions = measure_text(&score_text, Some(&font), score_font_size, 1.0);
        let score_y = title_y + 120.0;

        draw_text_ex(
            &score_text,
            center_x - (score_dimensions.width / 2.0),
            score_y,
            TextParams {
                font: Some(&font),
                font_size: score_font_size,
                color: WHITE,
                ..Default::default()
            },
        );

        // Calculate button positions
        let buttons_y = score_y + 120.0;
        let total_width = BUTTON_WIDTH * 2.0 + BUTTON_GAP;
        let retry_button_x = center_x - (total_width / 2.0);
        let quit_button_x = retry_button_x + BUTTON_WIDTH + BUTTON_GAP;

        let (mouse_x, mouse_y) = mouse_position();

        // Check if retry button is hovered or clicked
        let retry_hovered = mouse_x >= retry_button_x
            && mouse_x <= retry_button_x + BUTTON_WIDTH
            && mouse_y >= buttons_y
            && mouse_y <= buttons_y + BUTTON_HEIGHT;

        // Check if quit button is hovered or clicked
        let quit_hovered = mouse_x >= quit_button_x
            && mouse_x <= quit_button_x + BUTTON_WIDTH
            && mouse_y >= buttons_y
            && mouse_y <= buttons_y + BUTTON_HEIGHT;

        // Draw retry button
        let retry_color = if retry_hovered { GREEN } else { WHITE };
        draw_rectangle(
            retry_button_x,
            buttons_y,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            retry_color,
        );

        let retry_text = "RETRY";
        let retry_text_dimensions = measure_text(retry_text, Some(&font), 40, 1.0);
        draw_text_ex(
            retry_text,
            retry_button_x + (BUTTON_WIDTH - retry_text_dimensions.width) / 2.0,
            buttons_y + BUTTON_HEIGHT / 2.0 + retry_text_dimensions.height / 2.0,
            TextParams {
                font: Some(&font),
                font_size: 40,
                color: BLACK,
                ..Default::default()
            },
        );

        // Draw quit button
        let quit_color = if quit_hovered { RED } else { WHITE };
        draw_rectangle(quit_button_x, buttons_y, BUTTON_WIDTH, BUTTON_HEIGHT, quit_color);

        let quit_text = "QUIT";
        let quit_text_dimensions = measure_text(quit_text, Some(&font), 40, 1.0);
        draw_text_ex(
            quit_text,
            quit_button_x + (BUTTON_WIDTH - quit_text_dimensions.width) / 2.0,
            buttons_y + BUTTON_HEIGHT / 2.0 + quit_text_dimensions.height / 2.0,
            TextParams {
                font: Some(&font),
                font_size: 40,
                color: BLACK,
                ..Default::default()
            },
        );

        // Check for mouse clicks
        if is_mouse_button_pressed(MouseButton::Left) {
            if retry_hovered {
                return GameOverAction::Retry;
            } else if quit_hovered {
                return GameOverAction::Quit;
            }
        }

        next_frame().await;
    }
}

