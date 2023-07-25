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
use block::{Block, init_blocks, BlockType};
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
    let mut balls: Vec<Ball> = vec![Ball::new(
        Vec2::new(screen_width() * 0.5f32, screen_height() * 0.5f32)
    )];

    let mut blocks: Vec<Block> = Vec::new();
    
    init_blocks(&mut blocks);

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

                let mut spawn_later: Vec<Ball> = Vec::new();
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
                            if block.lives <= 0 {
                                score += 10;

                                if block.block_type == BlockType::SpawnBallOnDeath {
                                    spawn_later.push(Ball::new(
                                        ball.rect.point() + Vec2::new(0f32, 20f32)
                                    ));
                                }
                            }
                        }
                    }
                }

                for ball in spawn_later.into_iter() {
                    balls.push(ball);
                } 

                let balls_count = balls.len();
                balls.retain(|ball| ball.rect.y < screen_height());
                let removed_balls = balls_count - balls.len();

                if removed_balls > 0 && balls.is_empty() {
                    player.lives -= 1;

                    if player.lives <= 0 {
                        current_state = GameState::Dead;
                    } else {
                        balls.push(Ball::new(
                            player.rect.point() + Vec2::new(
                                player.rect.w / 2f32,
                                -80f32
                            )
                        ))
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
                    reset(&mut score, &mut player, &mut blocks, &mut balls);
                }

                draw_title_text(&format!("You Win! {} score", score), font)
            },
            GameState::Dead => {
                if is_key_pressed(KeyCode::Space) {
                    current_state = GameState::Menu;
                    reset(&mut score, &mut player, &mut blocks, &mut balls);
                }

                draw_title_text(&format!("You Lose! {} score", score), font)
            },
        }

        next_frame().await
    }
}

fn reset(
    score: &mut i32,
    player: &mut Player,
    blocks: &mut Vec<Block>,
    balls: &mut Vec<Ball>
) {
    *player = Player::new();
    *score = 0;
    balls.clear();
    *balls = vec![Ball::new(
        Vec2::new(screen_width() * 0.5f32, screen_height() * 0.5f32)
    )];
    init_blocks(blocks);
}
