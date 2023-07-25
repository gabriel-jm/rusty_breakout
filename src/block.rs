use macroquad::prelude::*;

pub const BLOCK_SIZE: Vec2 = Vec2::from_array([100f32, 30f32]);

#[derive(PartialEq)]
pub enum BlockType {
    Regular,
    SpawnBallOnDeath
}

pub struct Block {
    pub rect: Rect,
    pub lives: u16,
    pub block_type: BlockType
}

impl Block {
    pub fn new(position: Vec2, block_type: BlockType) -> Self {
        Block {
            lives: 2,
            block_type,
            rect: Rect::new(
                position.x,
                position.y,
                BLOCK_SIZE.x,
                BLOCK_SIZE.y
            )
        }
    }

    pub fn draw(&self) {
        let color = match self.block_type {
            BlockType::Regular => {
                match self.lives {
                    2 => RED,
                    _ => ORANGE
                }
            },
            BlockType::SpawnBallOnDeath => GREEN
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

pub fn init_blocks(blocks: &mut Vec<Block>) {
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
        let random_chance = rand::gen_range(0, 100);

        blocks.push(
            Block::new(
                board_start_position + Vec2::new(block_x, block_y),
                if random_chance > 10 {
                    BlockType::Regular
                } else {
                    BlockType::SpawnBallOnDeath
                }
            )
        )
    }
}
