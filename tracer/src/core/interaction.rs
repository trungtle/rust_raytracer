use math::{Float, Vec2, Vec3};

use crate::core::primitive::Primitive;

pub struct SurfaceInteraction {
    pub t: Float,
    pub hit_point: Vec3,
    pub hit_normal: Vec3,
    pub hit_uv: Vec2,
    pub hit_primitive: Option<Primitive>,
    pub hit_front_face: bool
}

impl SurfaceInteraction {
    pub fn new() -> Self {
        Self {
            t: -1.,
            hit_point: Vec3::from(0.),
            hit_normal: Vec3::from(0.),
            hit_uv: Vec2::from(0.),
            hit_primitive: Option::None,
            hit_front_face: true
        }
    }
}