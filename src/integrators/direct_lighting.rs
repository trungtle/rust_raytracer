use crate::camera::PerspectiveCamera;
use crate::core::{
    Spectrum,
    View
};
use crate::integrators::Integrator;
use crate::math:: {
    Sampler,
    Vec2,
    Vec3
};
use crate::ray::Ray;

trait Draw {
    fn draw(&self);
}

trait Hitable {
    fn hit(&self, ray: &Ray) -> SurfaceInteraction;
}

pub trait DrawHitable: Draw + Hitable {}
impl<T: Draw + Hitable> DrawHitable for T {}

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
}

impl Draw for Sphere { 
    fn draw(&self) {
        
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray) -> SurfaceInteraction {

    	// Sphere equation
		// (x - cx)^2 + (y - cy)^2 + (z - cz)^2 = R^2
		// or dot((p - c), (p - c)) - R^2 = 0
		// p(t) = origin + t * direction
		// Solve for this equation using quadratic formula
		// t = -b +/- sqrt(b*b - 4*a*c) / 2 * a

        let mut intersect = SurfaceInteraction::new();
		let oc = ray.origin;
		let a = Vec3::dot(ray.direction, ray.direction);
		let b = 2. * Vec3::dot(ray.direction, oc);
		let c = Vec3::dot(oc, oc) - self.radius_sq;

        const T_MIN: f64 = 1e-3;
        const T_MAX: f64 = 10000.;

		let discriminant = b * b - 4. * a * c;
		if discriminant > 0. {
			let t = (-b - f64::sqrt(discriminant)) / (2. * a);

			if t > T_MIN && t < T_MAX {
				intersect.t = t;
				intersect.hit_point = ray.point_at(t);
				// intersect.hit_normal = N(intersect.P, ray.time);
			} else {
                let t = (-b + f64::sqrt(discriminant)) / (2. * a);
                if t > T_MIN && t < T_MAX
                {
                    intersect.t = t;
                    intersect.hit_point = ray.point_at(t);
                }    
            }
		}
        intersect
    }
}

pub struct Scene {
    pub drawables: Vec<Box<dyn DrawHitable>>,
    pub persp_camera: PerspectiveCamera,
}

impl Scene {
    pub fn new(persp_camera: PerspectiveCamera) -> Self {
        Self {
            drawables: vec![Box::new(Sphere::new(Vec3::from(0.), 3.))],
            persp_camera
        }
    }
}

impl Draw for Scene {
    fn draw(&self) {
        for drawable in self.drawables.iter() {
            drawable.draw();
        }
    }
}

impl Hitable for Scene {
    fn hit(&self, ray: &Ray) -> SurfaceInteraction {

        let mut closest_t = 99999.;
        let mut closest_hit = SurfaceInteraction::new();
        for drawable in self.drawables.iter() {
            let hit = drawable.hit(&ray);
            if hit.t > 0. && hit.t < closest_t {
                closest_t = hit.t;
                closest_hit = hit;
            }
        }

        closest_hit
    }
}

struct SurfaceInteraction {    
    pub t: f64,
    pub hit_point: Vec3,
    pub hit_normal: Vec3
}

impl SurfaceInteraction {
    pub fn new() -> Self {
        Self {
            t: -1.,
            hit_point: Vec3::from(0.),
            hit_normal: Vec3::from(0.)
        }
    }
}

pub struct DirectLightingIntegrator {
    pub scene: Scene,
}

impl DirectLightingIntegrator {
    pub fn new(scene: Scene) -> Self {
        Self {
            scene
        }
    }

    fn li(&self, ray: &Ray) -> Spectrum {
        const MAX_DEPTH: u32 = 1;
        let mut acc_color = Spectrum::ColorRGB(Vec3::from(0.));
        
        for _ in 0..MAX_DEPTH {
            let interaction = self.scene.hit(ray);
            if interaction.t > 0. {
                acc_color = Spectrum::ColorRGB(Vec3::new(1., 0., 0.));
            }
        }
        acc_color
    }
}

impl Integrator for DirectLightingIntegrator {
    fn render(&mut self, view: &View) -> Spectrum {

        let samples_per_pixel = 10;
        for y in (0..view.height).rev() {
            for x in 0..view.width {
                let Spectrum::ColorRGB(mut total_spectrum) = Spectrum::ColorRGB(Vec3::from(0.));
                for _ in 0..samples_per_pixel {
                    let uv: Vec2 = Sampler::sample_from_pixel(Vec2 {x: x as f64, y: y as f64}, view.width, view.height);
                    let ray = self.scene.persp_camera.get_ray(&uv);
                    let li = self.li(&ray);
                    match li {
                        Spectrum::ColorRGB(li) => total_spectrum = li + total_spectrum
                    }    
                }
                total_spectrum /= samples_per_pixel as f64;
                self.scene.persp_camera.write_to_film(x, y, Spectrum::ColorRGB(total_spectrum));
            }
        }
        
        self.scene.persp_camera.write_film_to_file();
        Spectrum::ColorRGB(Vec3::from(0.))
    }
}