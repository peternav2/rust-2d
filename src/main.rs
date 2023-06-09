use macroquad::prelude::*;

const BALL_SIZE: f32 = 25f32; 
const GRAVITY: f32 = -9.81f32;


struct Ball {
    circ: Circle,
    vel: Vec2,
}
impl Ball {
    pub fn new() -> Self {
        Self {
            circ: Circle::new(
                      screen_width() * 0.5f32 - BALL_SIZE * 0.5f32,
                      screen_height() * 0.5f32 - BALL_SIZE * 1f32,
                      BALL_SIZE,
                      ),
                      vel: Vec2::from_array([0f32,0f32]),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.circ.x += self.vel.x * dt;
        self.circ.y += self.vel.y * dt * GRAVITY;
        self.vel.y += GRAVITY * dt;

        if self.circ.x < 0f32 {
            self.vel.x = self.vel.x * -1f32;
        }
        if self.circ.y > screen_height() {
            self.vel.y = self.vel.y * -1f32;
        }
    }
    pub fn draw(&self) {
        draw_circle(self.circ.x, self.circ.y, self.circ.r, BLUE);
    }
    pub fn resolve_self_collision(&mut self,ball:&mut Ball) {
        
    }

}

#[macroquad::main("rust-2d")]
async fn main() {
    let mut balls:Vec<Ball> = Vec::new();

    loop {
        println!("time; {}", get_frame_time());
        if is_key_pressed(KeyCode::Space) {
            balls.push(Ball::new());
        }
           
        for ball in balls.iter_mut() {
            ball.update(get_frame_time());

        }
        for ball in balls.iter_mut() {

        }
        clear_background(WHITE);
        for ball in balls.iter() {
            ball.draw();
        } 
        next_frame().await;
    }
}
