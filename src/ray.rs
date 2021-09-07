use crate::math::Vec3;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, mut d: Vec3) -> Self {
        Self {
            origin,
            direction: d.normalize()
        }
    }

    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}