use macroquad::{
    color::{BLACK, WHITE},
    shapes::draw_rectangle,
    text::{Font, TextParams, draw_text_ex, measure_text},
    window::{clear_background, next_frame, screen_height, screen_width},
};

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
