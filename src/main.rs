use std::f32::consts::FRAC_1_SQRT_2;
use macroquad::prelude::*;

const FOREGROUND_COLOR: Color = WHITE;
const BACKGROUND_COLOR: Color = BLACK;

#[macroquad::main("pong")]
async fn main() {
    let mut ball = Ball::default();

    loop {
        clear_background(BACKGROUND_COLOR);
        
        ball.draw();
        ball.update();

        next_frame().await;
    }
}
struct Ball {
    boundary: Rect,
    velocity: Vec2,
}
impl Default for Ball {
    fn default() -> Self {
        return Ball {
            boundary: Rect {
                x: screen_width() / 2.0,
                y: screen_height() / 2.0,
                w: Ball::SIZE,
                h: Ball::SIZE,
            },
            velocity: Ball::INITIAL_VELOCITY,
        };
    }
}
impl Ball {
    pub const SIZE: f32 = 10.0;
    pub const SPEED: f32 = Ball::SIZE / 2.0;
    pub const INITIAL_VELOCITY: Vec2 =
        Vec2::new(-FRAC_1_SQRT_2 * Ball::SPEED, -FRAC_1_SQRT_2 * Ball::SPEED);

    fn apply_velocity(&mut self) {
        self.boundary.x += self.velocity.x;
        self.boundary.y += self.velocity.y;
    }
    fn keep_on_screen(&mut self) {
        if self.boundary.top() <= 0.0 || self.boundary.bottom() >= screen_height() {
            self.velocity.y = -self.velocity.y;
        }
        if self.boundary.left() <= 0.0 || self.boundary.right() >= screen_width() {
            self.velocity.x = -self.velocity.x;
        }
    }
    pub fn update(&mut self) {
        self.apply_velocity();
        self.keep_on_screen();
    }
    
    pub fn draw(&self) {
        draw_rectangle(self.boundary.x, self.boundary.y, self.boundary.w, self.boundary.h, FOREGROUND_COLOR);
    }
}
