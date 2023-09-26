use crate::point::Point;
use crate::stick::Stick;
use macroquad::prelude::*;
const SPRING_CONSTANT: f32 = 30.0; // Adjust for the desired stiffness.
const DAMPING_CONSTANT: f32 = 0.10; // Start lower and increase if necessary.
const TIME_STEP: f32 = 0.1; // Smaller time steps can improve stability.
const GRAVITY: Vec2 = vec2(0.0, 9.81); // Assuming downward Y-axis.

pub struct Cloth {
    width: usize,
    height: usize,
    spacing: f32,
    points: Vec<Vec<Point>>,
    sticks: Vec<Stick>,
}
impl Cloth {
    pub fn new(width: usize, height: usize, spacing: f32) -> Cloth {
        let mut points = Vec::new();
        let mut sticks = Vec::new();
        let center = vec2(
            screen_width() / 2.0 - (width as f32 * spacing) / 2.0,
            screen_height() / 2.0 - (height as f32 * spacing) / 2.0,
        );
        for y in 0..height {
            let mut row = Vec::new();
            for x in 0..width {
                let pinned = y == 0 && x % 2 == 0;
                row.push(Point::new(
                    (x as f32 * spacing) + center.x,
                    (y as f32 * spacing) + (center.y),
                    1.0,
                    pinned,
                ));
            }
            points.push(row);
        }
        for y in 0..height {
            for x in 0..width {
                if x < width - 1 {
                    sticks.push(Stick {
                        p1_idx: (x, y),
                        p2_idx: (x + 1, y),
                        rest_length: spacing,
                    });
                }
                if y < height - 1 {
                    sticks.push(Stick {
                        p1_idx: (x, y),
                        p2_idx: (x, y + 1),
                        rest_length: spacing,
                    });
                }
            }
        }
        Cloth {
            width,
            height,
            points,
            sticks,
            spacing,
        }
    }
    pub fn update(&mut self) {
        for stick in &self.sticks {
            let point_a = &self.points[stick.p1_idx.1][stick.p1_idx.0];
            let point_b = &self.points[stick.p2_idx.1][stick.p2_idx.0];

            let displacement = point_a.pos - point_b.pos;
            let displacement_length = displacement.length();

            let direction = if displacement_length != 0.0 {
                displacement / displacement_length
            } else {
                vec2(0.0, 0.0) // or some other fallback value
            };

            let force_magnitude = -SPRING_CONSTANT * (displacement_length - stick.rest_length);
            let force = direction * force_magnitude;

            {
                let mut point_a_mut = &mut self.points[stick.p1_idx.1][stick.p1_idx.0];
                if !point_a_mut.pinned {
                    point_a_mut.acc += force;
                }
            }
            {
                let mut point_b_mut = &mut self.points[stick.p2_idx.1][stick.p2_idx.0];

                if !point_b_mut.pinned {
                    point_b_mut.acc -= force;
                }
            }
        }
        // Now, iterate through each point to apply gravity, damping, and perform the Verlet integration
        for y in 0..self.height {
            for x in 0..self.width {
                let mut point = &mut self.points[y][x];

                if !point.pinned {
                    point.acc += GRAVITY; // Add gravity

                    let velocity = (point.pos - point.prev_pos) / TIME_STEP;
                    let damping_force = -DAMPING_CONSTANT * velocity;
                    point.acc += damping_force; // Add damping

                    // Verlet integration
                    let new_position =
                        2.0 * point.pos - point.prev_pos + point.acc * TIME_STEP * TIME_STEP;

                    point.prev_pos = point.pos;
                    point.pos = new_position;

                    // Reset acceleration for next iteration
                    point.acc = vec2(0.0, 0.0);
                }
            }
        }
        for stick in &self.sticks {
            let point_a = &self.points[stick.p1_idx.1][stick.p1_idx.0];
            let point_b = &self.points[stick.p2_idx.1][stick.p2_idx.0];

            let delta = point_b.pos - point_a.pos;
            let delta_length = delta.length();
            let difference = if delta_length != 0.0 {
                (delta_length - stick.rest_length) / delta_length
            } else {
                0.0
            };

            let adjustment = delta * 0.5 * difference;

            let point_a = &mut self.points[stick.p1_idx.1][stick.p1_idx.0];
            if !point_a.pinned {
                point_a.pos += adjustment;
            }

            let point_b = &mut self.points[stick.p2_idx.1][stick.p2_idx.0];
            if !point_b.pinned {
                point_b.pos -= adjustment;
            }
        }
    }
    pub fn draw(&self) {
        //draw points
        for row in &self.points {
            for point in row {
                point.draw();
            }
        }
        //draw sticks
        for stick in &self.sticks {
            stick.draw(&self.points);
        }
    }
    pub fn cut_stick(&mut self, mouse_pos: Vec2) {
        //classic for loop because we need to remove elements
        //from the vector
        for i in 0..self.sticks.len() {
            let stick = &self.sticks[i];
            let p1 = &self.points[stick.p1_idx.1][stick.p1_idx.0];
            let p2 = &self.points[stick.p2_idx.1][stick.p2_idx.0];
            let distance = self.distance_to_segment(mouse_pos, p1, p2);
            let threshold = 5.0;
            if distance < threshold {
                //remove stick
                self.sticks.remove(i);
                //remove stick from points
                break;
            }
        }
    }
    fn distance_to_segment(&self, point: Vec2, endpoint1: &Point, endpoint2: &Point) -> f32 {
        let v = endpoint2.pos - endpoint1.pos;
        let w = point - endpoint1.pos;
        let t = w.dot(v) / v.length_squared().max(std::f32::EPSILON);

        if t < 0.0 {
            return (point - endpoint1.pos).length();
        } else if t > 1.0 {
            return (point - endpoint2.pos).length();
        }

        let projected_point = endpoint1.pos + v * t;
        (point - projected_point).length()
    }
}
