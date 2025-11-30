use math::Float;
use math::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Vec3::from(0.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
        }
    }
}

impl Ray {
    pub fn new(origin: Vec3, d: Vec3) -> Self {
        Self {
            origin,
            direction: d.normalize(),
        }
    }

    pub fn point_at(&self, t: Float) -> Vec3 {
        self.origin + t * self.direction
    }
}
