use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([140f32, 30f32]);
const PLAYER_SPEED: f32 = 700f32;

pub struct Player {
    pub rect: Rect,
    pub lives: u16
}

impl Player {
    pub fn new() -> Self {
        Player {
            lives: 3,
            rect: Rect::new(
                screen_width() * 0.5 - PLAYER_SIZE.x * 0.5,
                screen_height() - 100.0,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y
            )
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut x_speed = 0.0;

        if is_key_down(KeyCode::Left) {
            x_speed -= 1.0;
        }

        if is_key_down(KeyCode::Right) {
            x_speed += 1.0;
        }

        self.rect.x += x_speed * dt * PLAYER_SPEED;

        if self.rect.x < 0.0 {
            self.rect.x = 0.0;
        }

        if self.rect.x + self.rect.w > screen_width() {
            self.rect.x = screen_width() - self.rect.w;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            DARKGRAY
        );
    }
}
