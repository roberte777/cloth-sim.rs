use crate::force_funcs::damping_force;
use crate::point::Point;
use crate::stick::Stick;
use macroquad::prelude::*;
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
                    (y as f32 * spacing) + (center.y / 2.5),
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
        //update points

        for row in &mut self.points {
            for point in row {
                point.update();
            }
        }
        for stick in &mut self.sticks {
            stick.update(&mut self.points);
        }

        //damping force needs to be applied last because
        //it is dependent on the velocity of the points
        //after the spring force has been applied
        for row in &mut self.points {
            for point in row {
                point.apply_force(damping_force(point));
            }
        }

        for row in &mut self.points {
            for point in row {
                point.apply();
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
