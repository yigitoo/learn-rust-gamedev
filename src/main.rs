use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150_f32, 40_f32]);
const PLAYER_SPEED: f32 = 700_f32;
const BLOCK_SIZE: Vec2 = Vec2::from_array([150_f32, 40_f32]);
const BALL_SIZE: f32 = 50f32;
const BALL_SPEED: f32 = 400f32;

pub struct Player {
    rect: Rect,
    color: Color
}

impl Player {
    pub fn new(color: Color) -> Self {
        Self {
            rect: Rect::new(
                    screen_width() * 0.5_f32,
                    screen_height() - 100_f32,
                    PLAYER_SIZE.x,
                    PLAYER_SIZE.y,
                ),
            color: color
        }
    }

    pub fn update(&mut self, dt: f32) {
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1_f32,
            (false, true) => 1_f32,
            _ => 0_f32,
        };

        self.rect.x += x_move * dt * PLAYER_SPEED;
        if self.rect.x < 0_f32 {
            self.rect.x = 0_f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
        
    } 
    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.color,    
        );
    }
}

pub struct Block {
    rect: Rect,
    color: Color
}

impl Block {
    pub fn new(pos: Vec2, colour: Color) -> Self {
        Self {
            rect: Rect::new(
                pos.x,
                pos.y,
                BLOCK_SIZE.x,
                BLOCK_SIZE.y,
            ),
            color: colour
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.color,
        );
    }
}

pub struct Ball {
    rect: Rect,
    vel: Vec2,
    color: Color
}

impl Ball {
    pub fn new(pos: Vec2, color: Color) -> Self {
        Self {
            rect: Rect::new(),
            vel: vec2(rand::gen_range(-1_f32, 1_f32), 1_f32).normalize(),
            color: color,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;

        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1_f32;
        }
        if self.rect.y < 0_f32 {
            self.vel.y = 1_f32;
        }
    }
}

#[macroquad::main("Atari Breakout")]
async fn main() {
    let mut player = Player::new(WHITE);
    let mut blocks = Vec::new();
    let mut balls = Vec::new();

    let (width, height) = (4, 4);
    let padding = 10_f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2((screen_width() - (total_block_size.x * width as f32))*0.5_f32, 50_f32);
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / height) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y), RED));
    }

    loop {
        player.update(get_frame_time());
        clear_background(BLACK);

        player.draw();
        for block in blocks.iter() {
            block.draw();
        }

        next_frame().await
    }
}
