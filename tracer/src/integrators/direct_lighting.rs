use std::f64::consts::PI;

use rayon::prelude::*;

use crate::core::{
    interaction::SurfaceInteraction,
    material::Material,
    primitive::Primitive,
    ray::Ray,
    sampler::Sampler,
    scene::Scene,
    spectrum::Spectrum,
    view::View
};

use crate::math::vectors::{Vec2,Vec3};

pub struct DirectLightingIntegrator {
    pub scene: Scene,
}

impl DirectLightingIntegrator {
    pub fn new(scene: Scene) -> Self {
        Self {
            scene
        }
    }

    fn brdf_lambert(&self, spectrum: Spectrum) -> Spectrum
    {
        return spectrum / PI;
    }

    fn li(&self, ray: &Ray) -> Spectrum {
        const MAX_DEPTH: u32 = 5;

        let mut acc_color = Spectrum::ColorRGB(Vec3::from(1.0));
        let mut scatter_ray = ray.clone();

        for _depth in 0..MAX_DEPTH {
            let mut isect = SurfaceInteraction::new();
            let hit = self.scene.intersect(&scatter_ray, &mut isect);
            if !hit {
                acc_color = acc_color * (self.scene.environment_light)(&ray);
                break;
            }

            let mut material_color = Spectrum::ColorRGB(Vec3::new(1.0,1.0,1.0));

            match isect.hit_primitive {
                Some(primitive) => {
                    match *primitive {
                        Primitive::Shape(primitive) => {
                            match primitive.material {
                                Some(material) => {
                                    match *material {
                                        Material::Constant(material) => {
                                            material.scatter(&mut scatter_ray, &mut material_color, &isect.hit_normal);
                                        },
                                        Material::Matte(_material) => {

                                        },
                                        Material::Metal(material) => {
                                            material_color = material.color.clone();
                                            material.scatter(&mut scatter_ray, &mut material_color, &isect.hit_point, &isect.hit_normal);
                                        }
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                },
                None => {}
            }

            // New ray
            if scatter_ray.direction.near_zero() {
                scatter_ray.direction = isect.hit_normal;
            }
            scatter_ray.origin = isect.hit_point + scatter_ray.direction * 1e-3;

            let n_dot_l = f64::clamp(Vec3::dot(isect.hit_normal, scatter_ray.direction), 0., 1.);

            //acc_color = acc_color * self.brdf_lambert(material_color) * n_dot_l * 2.0 * PI;
            acc_color = acc_color * material_color;
        }
        Spectrum::ColorRGB(acc_color.clamp(0., 1.))
    }
}

impl DirectLightingIntegrator {
    pub fn render(&mut self, view: &View) {
        let samples_per_pixel = view.samples_per_pixel;
        let single_thread = true;

        let num_pixels = view.width as usize * view.height as usize;
        let mut pixels = vec![Spectrum::ColorRGB(Vec3::from(0.)); num_pixels];

        if single_thread {
            // Single threaded version
            for y in (0..view.height).rev() {
                for x in 0..view.width {
                    let Spectrum::ColorRGB(mut total_spectrum) = Spectrum::ColorRGB(Vec3::from(0.));
                    for _ in 0..samples_per_pixel {
                        let uv: Vec2 = Sampler::sample_from_pixel(Vec2 {x: x as f64, y: y as f64}, view.width, view.height);
                        let mut ray = self.scene.persp_camera.get_ray(&uv);
                        let li = self.li(&mut ray);
                        match li {
                            Spectrum::ColorRGB(li) => total_spectrum = li + total_spectrum
                        }
                    }
                    total_spectrum /= samples_per_pixel as f64;
                    // Gamma correction
                    total_spectrum = Vec3::sqrt(total_spectrum);
                    let i: usize = (y * view.width + x) as usize;
                    pixels[i] = Spectrum::ColorRGB(total_spectrum);
                }
            }
        } else {
            // Parallel version
            pixels.par_iter_mut().enumerate().for_each(|(i, pixel)| {
                let x: u32 = i as u32 % view.width;
                let y: u32 = i as u32 / view.width;
                let mut total_spectrum = (0..samples_per_pixel).into_par_iter().
                map(|_sample| {
                    let uv: Vec2 = Sampler::sample_from_pixel(Vec2 {x: x as f64, y: y as f64}, view.width, view.height);

                    let ray = self.scene.persp_camera.get_ray(&uv);
                    let Spectrum::ColorRGB(color) = self.li(&ray);
                    color
                }).sum::<Vec3>();

                total_spectrum /= samples_per_pixel as f64;
                // Gamma correction
                total_spectrum = Vec3::sqrt(total_spectrum);
                *pixel = Spectrum::ColorRGB(total_spectrum);
            });

        }
        self.scene.persp_camera.set_pixels(pixels);
        self.scene.persp_camera.write_film_to_file();

    }
}