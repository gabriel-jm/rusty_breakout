mod player;
mod block;
mod ball;
mod collision;
mod game_state;
mod text;

use ball::Ball;
use collision::has_collide;
use game_state::GameState;
use macroquad::prelude::*;
use player::Player;
use block::{Block, BLOCK_SIZE};
use text::draw_title_text;

#[macroquad::main("Breakout")]
async fn main() {
    let mut current_state = GameState::Menu;
    let font = load_ttf_font("resources/Heebo-VariableFont_wght.ttf")
        .await
        .unwrap()
    ;

    let mut score = 0;

    let mut player = Player::new();
    let mut balls: Vec<Ball> = vec![
        Ball::new(
            Vec2::new(screen_width() * 0.5f32, screen_height() * 0.5f32)
        )
    ];
    
    let mut blocks: Vec<Block> = Vec::new();
    
    let block_width = 6;
    let padding = 5f32;

    let total_block_size = BLOCK_SIZE + Vec2::new(padding, padding);
    let board_start_position = Vec2::new(
        (screen_width() - (total_block_size.x * block_width as f32)) * 0.5f32,
        50f32
    );

    for i in 0..block_width * block_width {
        let block_x = (i % block_width) as f32 * total_block_size.x;
        let block_y = (i / block_width) as f32 * total_block_size.y;

        blocks.push(
            Block::new(
                board_start_position + Vec2::new(block_x, block_y)
            )
        )
    }

    loop {
        clear_background(WHITE);

        player.draw();

        for block in blocks.iter() {
            block.draw();
        }

        for ball in balls.iter() {
            ball.draw();
        }

        match current_state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) {
                    current_state = GameState::Game;
                }

                draw_title_text("Press START to start", font);
            },
            GameState::Game => {
                player.update(get_frame_time());

                for ball in balls.iter_mut() {
                    ball.update(get_frame_time());
                    
                    if let Some(col) = has_collide(&ball.rect, &player.rect) {
                        let (intersection, direction) = col;
                        ball.repel(&intersection, &direction);
                    }

                    for block in blocks.iter_mut() {
                        if let Some(col) = has_collide(&ball.rect, &block.rect) {
                            let (intersection, direction) = col;
                            ball.repel(&intersection, &direction);
                            block.lives -= 1;
                            score += 10;
                        }
                    }
                }

                let balls_count = balls.len();
                let is_last_ball = balls_count == 1;
                balls.retain(|ball| ball.rect.y < screen_height());
                let removed_balls = balls_count - balls.len();

                if removed_balls > 0 && is_last_ball {
                    player.lives -= 1;

                    if player.lives <= 0 {
                        current_state = GameState::Dead;
                    }
                }

                blocks.retain(|block| block.lives > 0);

                if blocks.is_empty() {
                    current_state = GameState::LevelCompleted;
                }

                let score_text = format!("Score: {}", score);
                let score_text_measure = measure_text(
                    &score_text,
                    Some(font),
                    30,
                    1.0
                );
                
                draw_text_ex(
                    &score_text,
                    screen_width() * 0.5 - score_text_measure.width * 0.5,
                    30.0,
                    TextParams {
                        font,
                        font_size: 28,
                        color: BLACK,
                        ..Default::default()
                    }
                );
                draw_text_ex(
                    &format!("Lives: {}", player.lives),
                    20.0,
                    30.0,
                    TextParams {
                        font,
                        font_size: 28,
                        color: BLACK,
                        ..Default::default()
                    }
                );
            },
            GameState::LevelCompleted => {
                if is_key_pressed(KeyCode::Space) {
                    current_state = GameState::Menu;
                }

                draw_title_text(&format!("You Win! {} score", score), font)
            },
            GameState::Dead => {
                if is_key_pressed(KeyCode::Space) {
                    current_state = GameState::Menu;
                }

                draw_title_text(&format!("You Lose! {} score", score), font)
            },
        }

        next_frame().await
    }
}

fn reset() {
    
}
