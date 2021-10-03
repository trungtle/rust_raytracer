use crate::math::{Vec2, Vec3};

pub struct SurfaceInteraction {    
    pub t: f64,
    pub hit_point: Vec3,
    pub hit_normal: Vec3,
    pub hit_uv: Vec2
}

impl SurfaceInteraction {
    pub fn new() -> Self {
        Self {
            t: -1.,
            hit_point: Vec3::from(0.),
            hit_normal: Vec3::from(0.),
            hit_uv: Vec2::from(0.)
        }
    }
}