use math::{Vec2, Vec3};

use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        Self {
            v0, v1, v2
        }
    }

    pub fn normal_at(&self, _point: &Vec3) -> Vec3 {
        let v1v0 = self.v1 - self.v0;
        let v2v0 = self.v2 - self.v0;
        Vec3::cross(v2v0, v1v0)
    }

    pub fn intersect(&self, ray: &Ray, isect: &mut SurfaceInteraction) -> bool {
        // Ref: https://www.shadertoy.com/view/MlGcDz

        let v1v0 = self.v1 - self.v0;
        let v2v0 = self.v2 - self.v0;
        let rov0 = ray.origin - self.v0;

        let n = Vec3::cross( v1v0, v2v0 );
        let q = Vec3::cross( rov0, ray.direction );
        let d = 1.0 / Vec3::dot( ray.direction, n );
        let u = d*Vec3::dot( -q, v2v0 );
        let v = d*Vec3::dot(  q, v1v0 );
        let t = d*Vec3::dot( -n, rov0 );

        if u < 0.0 || v < 0.0 || (u + v) > 1.0 {
            return false;
        }

        isect.t = t;
        isect.hit_point = Vec3 { x: t, y: u, z: v };
        isect.hit_normal = self.normal_at(&isect.hit_point);
        isect.hit_uv = Vec2 { 0: u, 1: v };
        return true;
    }
}
