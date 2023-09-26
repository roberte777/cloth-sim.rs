use crate::force_funcs::gravity;
use macroquad::prelude::*;
#[derive(Clone)]
pub struct Point {
    pub prev_pos: Vec2,
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub pinned: bool,
}
impl Point {
    pub fn new(x: f32, y: f32, mass: f32, pinned: bool) -> Point {
        Point {
            pos: vec2(x, y),
            prev_pos: vec2(x, y),
            vel: vec2(0.0, 0.0),
            acc: vec2(0.0, 0.0),
            mass,
            pinned,
        }
    }
    pub fn apply_force(&mut self, force: Vec2) {
        self.acc += force / self.mass;
    }

    pub fn update(&mut self) {
        if self.pinned {
            return;
        }
        self.apply_force(gravity());
    }

    pub fn apply(&mut self) {
        if self.pinned {
            return;
        }
        //use verlet integration
        // x = 2x - prev_x + a * dt^2
        //cap out acceleration
        self.acc = self.acc.clamp_length_max(4.0);
        let time = 0.7;
        // update position
        //self.pos += self.vel * time;
        let new_pos = 2. * self.pos - self.prev_pos + self.acc * time * time;
        self.prev_pos = self.pos;
        self.pos = new_pos;
        // update velocity, used in damping force
        self.vel += self.acc * time;
        // cap out velocity
        self.vel = self.vel.clamp_length_max(10.0);
        self.acc *= 0.0;
    }
    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 2.0, WHITE);
    }
}
