use std::f64::consts::PI;

use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;
use crate::math::vectors::{Vec2, Vec3};

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    radius_sq: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
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
		let phi = f64::atan2(point.z, point.x);
		let theta = f64::asin(point.y);
		let u = 1. - (phi + PI) / (2. * PI);
		let v = (theta + PI / 2.) / PI;
		Vec2 {x: u, y: v}
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

        const T_MIN: f64 = 1e-3;
        const T_MAX: f64 = 10000.;

		let discriminant = b * b - 4. * a * c;
		if discriminant > 0. {
			let t = (-b - f64::sqrt(discriminant)) / (2. * a);
			if t > T_MIN && t < T_MAX {
				isect.t = t;
				isect.hit_point = ray.point_at(t);
				isect.hit_normal = self.normal_at(&isect.hit_point);
                isect.hit_uv = self.uv_at(&isect.hit_point);
                return true;
			} else {
                let t = (-b + f64::sqrt(discriminant)) / (2. * a);
                if t > T_MIN && t < T_MAX
                {
                    isect.t = t;
                    isect.hit_point = ray.point_at(t);
                    isect.hit_normal = self.normal_at(&isect.hit_point);
                    isect.hit_uv = self.uv_at(&isect.hit_point);
                    return true;
                }    
            }
		}
        return false;
    }
}
