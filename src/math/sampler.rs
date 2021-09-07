use rand::prelude::*;

pub use crate::math::Vec2;

pub struct Sampler {
}

impl Sampler {
    pub fn sample_from_pixel(point: Vec2, width: u32, height: u32) -> Vec2 {
        let ru: f64 = random();
        let rv: f64 = random();
        let u = (point.x + ru) / width as f64;
        let v = (point.y + ru) / height as f64;
        Vec2 { x: point.x, y: point.y}
    }

    pub fn sample_from_unit_disk() -> Vec2 {
        let mut point: Vec2 = 2. * Vec2::new(random(), random()) - Vec2::from(1.);
        loop {
            // dot product with itself is squared length
            if Vec2::dot(point, point) >= 1. {
                break;
            }
            point = 2. * Vec2::new(random(), random()) - Vec2::from(1.);
        }
        point
    }
}