use macroquad::{text::{Font, measure_text, draw_text_ex, TextParams}, window::{screen_width, screen_height}, prelude::BLACK};

pub fn draw_title_text(text: &str, font: Font) {
    let text_dimension = measure_text(
        text,
        Some(font),
        50,
        1.0
    );

    draw_text_ex(
        text,
        screen_width() * 0.5 - text_dimension.width * 0.5,
        screen_height() * 0.5 - text_dimension.height * 0.5,
        TextParams {
            font,
            font_size: 50,
            color: BLACK,
            ..Default::default()
        }
    );
}
