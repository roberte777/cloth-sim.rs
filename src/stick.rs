use crate::force_funcs::*;
use crate::point::Point;
use macroquad::prelude::*;
pub struct Stick {
    pub p1_idx: (usize, usize),
    pub p2_idx: (usize, usize),
    pub rest_length: f32,
}
impl Stick {
    pub fn update(&mut self, points: &mut Vec<Vec<Point>>) {
        let p1 = &points[self.p1_idx.1][self.p1_idx.0];
        let p2 = &points[self.p2_idx.1][self.p2_idx.0];
        let force = hookes_law(p1, p2, self.rest_length);
        points[self.p1_idx.1][self.p1_idx.0].apply_force(force);
        points[self.p2_idx.1][self.p2_idx.0].apply_force(-force);
    }
    pub fn draw(&self, points: &Vec<Vec<Point>>) {
        let p1 = &points[self.p1_idx.1][self.p1_idx.0];
        let p2 = &points[self.p2_idx.1][self.p2_idx.0];
        draw_line(p1.pos.x, p1.pos.y, p2.pos.x, p2.pos.y, 1.0, WHITE);
    }
}
