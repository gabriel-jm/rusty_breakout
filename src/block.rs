use macroquad::prelude::*;

pub const BLOCK_SIZE: Vec2 = Vec2::from_array([100f32, 30f32]);

pub struct Block {
    pub rect: Rect,
    pub lives: u16
}

impl Block {
    pub fn new(position: Vec2) -> Self {
        Block {
            lives: 2,
            rect: Rect::new(
                position.x,
                position.y,
                BLOCK_SIZE.x,
                BLOCK_SIZE.y
            )
        }
    }

    pub fn draw(&self) {
        let color = match self.lives {
            2 => RED,
            _ => ORANGE
        };

        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            color
        );
    }
}
