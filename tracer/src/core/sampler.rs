use rand::prelude::*;
use math::{Vec2, Vec3};

pub struct Sampler {
    pub rng_generator: ThreadRng
}

impl Sampler {
    pub fn new() -> Self {
        Self {
            rng_generator: rand::thread_rng()
        }
    }

    pub fn random_0_1(&mut self) -> f64 {
        self.rng_generator.sample(rand::distributions::Uniform::new(0f64, 1f64))
    }

    pub fn random_vec2_0_1(&mut self) -> Vec2 {
        let ru = self.rng_generator.gen_range(0f64..1f64);
        let rv = self.rng_generator.gen_range(0f64..1f64);
        return Vec2::new(ru, rv);
    }

    pub fn sample_from_pixel(&mut self, point: Vec2, width: u32, height: u32) -> Vec2 {
        let ru: f64 = self.random_0_1();
        let rv: f64 = self.random_0_1();
        let u = (point.x + ru) / width as f64;
        let v = (point.y + rv) / height as f64;
        Vec2 { x: u, y: v}
    }

    pub fn sample_unit_disk(&mut self) -> Vec2 {
        let mut point: Vec2 = 2. * Vec2::new(self.random_0_1(), self.random_0_1()) - Vec2::from(1.);
        loop {
            // dot product with itself is squared length
            if Vec2::dot(point, point) < 1. {
                break;
            }
            point = 2. * Vec2::new(self.random_0_1(), self.random_0_1()) - Vec2::from(1.);
        }
        point
    }

    pub fn sample_unit_disk_concentric(u: Vec2) -> Vec2 {
        let u_offset = 2. * u - Vec2::from(1.0);
        if u_offset.x == 0.0 && u_offset.y == 0.0 {
            return Vec2::from(0.);
        }

        let mut theta = 0.;
        let mut r = 0.;
        if u_offset.x.abs() > u_offset.y.abs() {
            r = u_offset.x;
            theta = std::f64::consts::FRAC_PI_4 * (u_offset.y / u_offset.x);
        } else {
            r = u_offset.y;
            theta = std::f64::consts::FRAC_PI_2 - std::f64::consts::FRAC_PI_4 * (u_offset.x / u_offset.y);
        }
        return r * Vec2::new(theta.cos(), theta.sin());
    }

    pub fn sample_from_unit_sphere(&mut self) -> Vec3 {
        let mut point = Vec3::new(self.random_0_1(), self.random_0_1(), self.random_0_1());
        loop
		{
            if Vec3::length2(&point) < 1. {
                break;
            }
			point = 2. * Vec3::new(self.random_0_1(), self.random_0_1(), self.random_0_1()) - Vec3::from(1.); // Scale to -1 , 1 range
		}

		point
    }

    // Find a random direction, cosine weighted, with z axis as normal
	// If we sample the variables with cosine weighted, then we can use
	// our pdf as cos(theta) / pi
    pub fn sample_cosine_direction(&mut self) -> Vec3 {
		// Sampling with 2 variables over a cosine weighted direction
		// r1 = Integral_0_phi(1 /(2 * PI)) -> phi = 2 * PI * r1
		// r2 = Integral_0_theta(2 * PI * f(t) * sin(t)) with f(t) = cos(theta) / PI
		// -> r2 = 1 - cos^2(theta) -> cos(theta) = sqrt(1 - r2)

        // let r1 = Sampler::random_0_1();
        // let r2 = Sampler::random_0_1();
        // let z = f64::sqrt(1. - r2); // this is cos(theta)
        // let phi = 2. * PI * r1;

        // let r2_sqrt = f64::sqrt(r2);
        // let x = f64::cos(phi) * r2_sqrt;
        // let y = f64::sin(phi) * r2_sqrt;
        // Vec3::new(x, y, z)

		// Malley's method: sample from concentric disk, then project upward
        let random = self.random_vec2_0_1();
		let r = Sampler::sample_unit_disk_concentric(random);
		let z: f64 = 0.0_f64.max(1.0 - r.x * r.x - r.y * r.y);
		return Vec3::new(r.x, r.y, z);
    }
}