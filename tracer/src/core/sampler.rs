use rand::prelude::*;
use math::{Float, Vec2, Vec3};

pub struct Sampler {
    pub rng_generator: ThreadRng
}

impl Default for Sampler {
    fn default() -> Self {
        Self {
            rng_generator: rand::thread_rng()
        }
    }
}

impl Sampler {
    pub fn random_0_1(&mut self) -> Float {
        self.rng_generator.sample(rand::distributions::Uniform::new(0., 1.))
    }

    pub fn random_vec2_0_1(&mut self) -> Vec2 {
        let ru = self.rng_generator.gen_range(0.0..1.0);
        let rv = self.rng_generator.gen_range(0.0..1.0);
        return Vec2 { 0: ru, 1: rv};
    }

    pub fn sample_from_pixel(&mut self, point: Vec2, width: u32, height: u32) -> Vec2 {
        let ru = self.random_0_1();
        let rv = self.random_0_1();
        let u = (point.0 + ru) / width as Float;
        let v = (point.1 + rv) / height as Float;
        Vec2 { 0: u, 1: v}
    }

    pub fn sample_unit_disk(&mut self) -> Vec2 {
        let mut point: Vec2 = 2. * Vec2 { 0: self.random_0_1(), 1: self.random_0_1()} - Vec2::from(1.);
        loop {
            // dot product with itself is squared length
            if Vec2::dot(point, point) < 1. {
                break;
            }
            point = 2. * Vec2 { 0: self.random_0_1(), 1: self.random_0_1()} - Vec2::from(1.);
        }
        point
    }

    pub fn sample_unit_disk_concentric(u: Vec2) -> Vec2 {
        let u_offset = 2. * u - Vec2::from(1.0);
        if u_offset.0 == 0.0 && u_offset.1 == 0.0 {
            return Vec2::from(0.);
        }

        let mut theta = 0. as Float;
        let mut r = 0. as Float;
        if u_offset.0.abs() > u_offset.1.abs() {
            r = u_offset.0;
            theta = std::f32::consts::FRAC_PI_4 * (u_offset.1 / u_offset.0);
        } else {
            r = u_offset.1;
            theta = std::f32::consts::FRAC_PI_2 - std::f32::consts::FRAC_PI_4 * (u_offset.0 / u_offset.1);
        }
        return r * Vec2 { 0: theta.cos(), 1: theta.sin()};
    }

    pub fn sample_from_unit_sphere(&mut self) -> Vec3 {
        let mut point = Vec3 { x: self.random_0_1(), y: self.random_0_1(), z: self.random_0_1() };
        loop
		{
            if Vec3::length2(&point) < 1. {
                break;
            }
			point = 2. * Vec3 {x: self.random_0_1(), y: self.random_0_1(), z: self.random_0_1() } - Vec3::from(1.); // Scale to -1 , 1 range
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
		let z = Float::max(0.0, 1.0 - r.x() * r.x() - r.y() * r.y());
		return Vec3 { x: r.x(), y: r.y(), z: z};
    }
}