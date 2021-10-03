use rand::prelude::*;
use std::f64::consts::PI;

pub use crate::math::vectors::{Vec2, Vec3};

pub struct Sampler {
}

impl Sampler {
    fn random_0_1() -> f64 {
        rand::thread_rng().gen_range(0f64..1f64)
    }

    pub fn sample_from_pixel(point: Vec2, width: u32, height: u32) -> Vec2 {
        let ru: f64 = Sampler::random_0_1();
        let rv: f64 = Sampler::random_0_1();
        let u = (point.x + ru) / width as f64;
        let v = (point.y + rv) / height as f64;
        Vec2 { x: u, y: v}
    }

    pub fn sample_from_unit_disk() -> Vec2 {
        let mut point: Vec2 = 2. * Vec2::new(Sampler::random_0_1(), Sampler::random_0_1()) - Vec2::from(1.);
        loop {
            // dot product with itself is squared length
            if Vec2::dot(point, point) < 1. {
                break;
            }
            point = 2. * Vec2::new(Sampler::random_0_1(), Sampler::random_0_1()) - Vec2::from(1.);
        }
        point
    }

    pub fn sample_from_unit_sphere() -> Vec3 {
        let mut point = Vec3::new(Sampler::random_0_1(), Sampler::random_0_1(), Sampler::random_0_1());
        loop
		{
            if Vec3::length2(&point) < 1. {
                break;
            }
			point = 2. * Vec3::new(Sampler::random_0_1(), Sampler::random_0_1(), Sampler::random_0_1()) - Vec3::from(1.); // Scale to -1 , 1 range
		}

		point
    }

    // Find a random direction, cosine weighted, with z axis as normal
	// If we sample the variables with cosine weighted, then we can use
	// our pdf as cos(theta) / pi
    pub fn sample_cosine_direction() -> Vec3 {
		// Sampling with 2 variables over a cosine weighted direction
		// r1 = Integral_0_phi(1 /(2 * PI)) -> phi = 2 * PI * r1
		// r2 = Integral_0_theta(2 * PI * f(t) * sin(t)) with f(t) = cos(theta) / PI
		// -> r2 = 1 - cos^2(theta) -> cos(theta) = sqrt(1 - r2)
		
        let r1 = Sampler::random_0_1();
        let r2 = Sampler::random_0_1();
        let z = f64::sqrt(1. - r2); // this is cos(theta)
        let phi = 2. * PI * r1;

        let r2_sqrt = f64::sqrt(r2);
        let x = f64::cos(phi) * r2_sqrt;
        let y = f64::sin(phi) * r2_sqrt;
        Vec3::new(x, y, z)

		// Malley's method: sample from concentric disk, then project upward
		// let r = SampleConentricDisk();
		// float z = glm::sqrt(std::max(0.0f, 1.0f - r.x * r.x - r.y * r.y));
		// return vec3(r.x, r.y, z);
    }
}