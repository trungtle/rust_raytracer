use std::f64::consts::PI;
use std::sync::Arc;

use rayon::prelude::*;

use math::{Vec2, Vec3};

use crate::core::{
    interaction::SurfaceInteraction,
    primitive::Primitive,
    ray::Ray,
    sampler::Sampler,
    scene::Scene,
    spectrum::Spectrum,
    view::View
};

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

    fn li(&self, ray: &Ray, sampler: &mut Sampler) -> Spectrum {
        const MAX_DEPTH: u32 = 10;

        let mut acc_color = Spectrum::ColorRGB(Vec3::from(0.0));
        let mut scatter_ray = ray.clone();

        for depth in 0..MAX_DEPTH {
            let mut isect = SurfaceInteraction::new();
            let hit = self.scene.intersect(&scatter_ray, &mut isect);
            if !hit {
                if depth == 0 {
                    acc_color = (self.scene.environment_light)(&ray);
                } else {
                    acc_color = acc_color * (self.scene.environment_light)(&ray);
                }
                break;
            }

            let mut material_color = Spectrum::ColorRGB(Vec3::new(1.0,1.0,1.0));

            match isect.hit_primitive {
                Some(primitive) => {
                    match primitive.material {
                        Some(material) => {
                            material_color = material.value().clone();
                            material.scatter(&mut scatter_ray, &mut material_color, &isect.hit_point, &isect.hit_normal, sampler);
                        }
                        None => {}
                    }
                },
                None => {}
            }

            // New ray
            scatter_ray.direction = scatter_ray.direction.normalize();
            scatter_ray.origin = isect.hit_point + scatter_ray.direction * 1e-3;

            let n_dot_l = f64::clamp(Vec3::dot(isect.hit_normal, scatter_ray.direction), 0., 1.);

            //acc_color = acc_color * self.brdf_lambert(material_color) * n_dot_l * 2.0 * PI;
            if depth == 0 {
                acc_color = material_color;
            } else {
                acc_color = acc_color * material_color;
            }
        }
        Spectrum::ColorRGB(acc_color.clamp(0., 1.))
    }
}

impl DirectLightingIntegrator {
    pub fn render(&mut self, view: &View) {
        let samples_per_pixel = view.samples_per_pixel;
        let single_thread = false;

        let num_pixels = view.width as usize * view.height as usize;
        let mut pixels = vec![Spectrum::ColorRGB(Vec3::from(0.)); num_pixels];

        if single_thread {
            let mut sampler = Sampler::new();

            // Single threaded version
            for y in (0..view.height).rev() {
                for x in 0..view.width {
                    let Spectrum::ColorRGB(mut total_spectrum) = Spectrum::ColorRGB(Vec3::from(0.));
                    for _ in 0..samples_per_pixel {
                        let uv: Vec2 = sampler.sample_from_pixel(Vec2 {x: x as f64, y: y as f64}, view.width, view.height);
                        let mut ray = self.scene.persp_camera.get_ray(&uv, &mut sampler);
                        let li = self.li(&mut ray, &mut sampler);
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
            // pixels.par_iter_mut().enumerate().for_each(|(i, pixel)| {
            //     let x: u32 = i as u32 % view.width;
            //     let y: u32 = i as u32 / view.width;
            //     let mut total_spectrum = (0..samples_per_pixel).into_par_iter().
            //     map(|_sample| {
            //         let mut sampler = Sampler::new();
            //         let uv: Vec2 = sampler.sample_from_pixel(Vec2 {x: x as f64, y: y as f64}, view.width, view.height);

            //         let ray = self.scene.persp_camera.get_ray(&uv, &mut sampler);
            //         let Spectrum::ColorRGB(color) = self.li(&ray, &mut sampler);
            //         color
            //     }).sum::<Vec3>();

            //     total_spectrum /= samples_per_pixel as f64;
            //     // Gamma correction
            //     total_spectrum = Vec3::sqrt(total_spectrum);
            //     *pixel = Spectrum::ColorRGB(total_spectrum);
            // });
            for i in 0..num_pixels {
                let x: u32 = i as u32 % view.width;
                let y: u32 = i as u32 / view.width;
                let mut total_spectrum = (0..samples_per_pixel).into_par_iter().
                map(|_sample| {
                    let mut sampler = Sampler::new();
                    let uv: Vec2 = sampler.sample_from_pixel(Vec2 {x: x as f64, y: y as f64}, view.width, view.height);

                    let ray = self.scene.persp_camera.get_ray(&uv, &mut sampler);
                    let Spectrum::ColorRGB(color) = self.li(&ray, &mut sampler);
                    color
                }).sum::<Vec3>();

                total_spectrum /= samples_per_pixel as f64;
                // Gamma correction
                total_spectrum = Vec3::sqrt(total_spectrum);
                pixels[i] = Spectrum::ColorRGB(total_spectrum);
            };

        }
        self.scene.persp_camera.set_pixels(pixels);
        self.scene.persp_camera.write_film_to_file();

    }
}