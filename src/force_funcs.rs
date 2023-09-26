use crate::point::Point;
use macroquad::prelude::*;
const GRAVITY: f32 = 3.5;
const SPRING_CONSTANT: f32 = 1.;
const DAMPING_CONSTANT: f32 = 0.2;
//functions to return forces
pub fn gravity() -> Vec2 {
    vec2(0.0, GRAVITY)
}
//v3
pub fn hookes_law(p1: &Point, p2: &Point, rest_length: f32) -> Vec2 {
    -SPRING_CONSTANT * (rest_length - (p2.pos - p1.pos))
}

//v2
// pub fn hookes_law(p1: &Point, p2: &Point, rest_length: f32) -> Vec2 {
//     // spring_force = -k * displacement
//     let displacement = p2.pos - p1.pos;
//     let displacement = displacement.normalize() * (rest_length - displacement.length());
//     let spring_force = displacement * -SPRING_CONSTANT;
//     spring_force
// }
//possibly check out the implementation here:
//http://web.archive.org/web/20070610223835/http://www.teknikus.dk/tj/gdc2001.htm
//this usses a much simpler method for calculating the spring force, but is not as accurate
// if using this method, also need to change the last two lines of stick update
// to these lines:
// let p1_mass = p1.mass.clone();
// let p2_mass = p2.mass.clone();
// points[self.p1_idx.1][self.p1_idx.0].apply_force(force * p1_mass);
// points[self.p2_idx.1][self.p2_idx.0].apply_force(-force * p2_mass);
// pub fn hookes_law(p1: &Point, p2: &Point, rest_length: f32) -> Vec2 {
//     let delta = p2.pos - p1.pos;
//     let delta_length = delta.length();
//     let diff = (delta_length - rest_length) / (delta_length * (p1.mass + p2.mass));
//     delta * diff * 2.
// }

pub fn damping_force(point: &Point) -> Vec2 {
    // damping_force = -c * relative_velocity
    let relative_velocity = point.vel;
    relative_velocity * -DAMPING_CONSTANT
}
