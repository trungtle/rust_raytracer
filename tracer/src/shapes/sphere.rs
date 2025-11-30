use math::{Float, Vec2, Vec3};
use std::f32::consts::PI;

use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: Float,
    radius_sq: Float,
}

impl Sphere {
    pub fn new(center: Vec3, radius: Float) -> Self {
        Self {
            center,
            radius,
            radius_sq: radius * radius,
        }
    }

    pub fn normal_at(&self, point: &Vec3) -> Vec3 {
        (*point - self.center).normalize()
    }

    // Get UV for a unit sphere
    pub fn uv_at(&self, point: &Vec3) -> Vec2 {
        let phi = Float::atan2(point.z, point.x);
        let theta = Float::asin(point.y);
        let u = 1. - (phi + PI) / (2. * PI);
        let v = (theta + PI / 2.) / PI;
        Vec2 { 0: u, 1: v }
    }

    pub fn intersect(&self, ray: &Ray, isect: &mut SurfaceInteraction) -> bool {
        // Sphere equation
        // (x - cx)^2 + (y - cy)^2 + (z - cz)^2 = R^2
        // or dot((p - c), (p - c)) - R^2 = 0
        // p(t) = origin + t * direction
        // Solve for this equation using quadratic formula
        // t = -b +/- sqrt(b*b - 4*a*c) / 2 * a

        let oc = ray.origin - self.center;
        let a = Vec3::dot(ray.direction, ray.direction);
        let b = 2. * Vec3::dot(ray.direction, oc);
        let c = Vec3::dot(oc, oc) - self.radius_sq;

        const T_MIN: Float = 1e-3;
        const T_MAX: Float = 10000.;

        let discriminant = b * b - 4. * a * c;
        if discriminant > 0. {
            let t = (-b - Float::sqrt(discriminant)) / (2. * a);
            if t > T_MIN && t < T_MAX {
                isect.t = t;
                isect.hit_point = ray.point_at(t);
                isect.hit_normal = self.normal_at(&isect.hit_point);
                isect.hit_uv = self.uv_at(&isect.hit_point);
                return true;
            } else {
                let t = (-b + Float::sqrt(discriminant)) / (2. * a);
                if t > T_MIN && t < T_MAX {
                    isect.t = t;
                    isect.hit_point = ray.point_at(t);
                    isect.hit_normal = self.normal_at(&isect.hit_point);
                    isect.hit_front_face = Vec3::dot(ray.direction, isect.hit_normal) < 0.;
                    isect.hit_uv = self.uv_at(&isect.hit_point);
                    return true;
                }
            }
        }
        return false;
    }

    pub fn world_bound(&self) -> crate::core::bounds::Bounds3f {
        let p_min = self.center - Vec3::new(self.radius, self.radius, self.radius);
        let p_max = self.center + Vec3::new(self.radius, self.radius, self.radius);
        crate::core::bounds::Bounds3f { p_min, p_max }
    }
}
