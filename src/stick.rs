use crate::force_funcs::*;
use crate::point::Point;
use macroquad::prelude::*;
#[derive(Debug)]
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
    pub fn involves_point(&self, x: usize, y: usize) -> bool {
        self.p1_idx == (x, y) || self.p2_idx == (x, y)
    }
    pub fn other_point(&self, x: usize, y: usize) -> (usize, usize) {
        if !self.involves_point(x, y) {
            panic!("Called other_point but point was not in stick");
        }
        if self.p1_idx == (x, y) {
            return self.p2_idx;
        } else {
            return self.p1_idx;
        }
    }
}
