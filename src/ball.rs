use macroquad::{prelude::*, rand};

const BALL_SIZE: f32 = 40f32;
const BALL_SPEED: f32 = 400f32;

pub struct Ball {
    pub rect: Rect,
    velocity: Vec2
}

impl Ball {
    pub fn new(position: Vec2) -> Self {
        Ball {
            rect: Rect {
                x: position.x,
                y: position.x,
                w: BALL_SIZE,
                h: BALL_SIZE
            },
            velocity: Vec2 {
                x: rand::gen_range(-1f32, 1f32),
                y: 1f32
            }.normalize()
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.velocity.x * dt * BALL_SPEED;
        self.rect.y += self.velocity.y * dt * BALL_SPEED;

        if self.rect.x < 0f32 {
            self.velocity.x = 1f32;
        }

        if self.rect.x + self.rect.w > screen_width() {
            self.velocity.x = -1f32;
        }

        if self.rect.y < 0f32 {
            self.velocity.y = 1f32;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            DARKGRAY
        )
    }

    pub fn repel(&mut self, intersection: &Rect, direction: &Vec2) {
        if intersection.w > intersection.h {
            self.rect.y -= direction.y * intersection.h;
            self.velocity.y = -direction.y * self.velocity.y.abs();
            return;
        }

        self.rect.x -= direction.x * intersection.w;
        self.velocity.x = -direction.x * self.velocity.x.abs();
    }
}
