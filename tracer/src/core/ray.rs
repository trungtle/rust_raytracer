use math::Vec3;
use math::Float;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, d: Vec3) -> Self {
        Self {
            origin,
            direction: d.normalize()
        }
    }

    pub fn point_at(&self, t: Float) -> Vec3 {
        self.origin + t * self.direction
    }
}