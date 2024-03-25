use std::f64::consts::PI;

use image::{GenericImageView, Pixel};
use rayon::prelude::*;
use log::info;

use math::{Float, Vec2, Vec3};

use crate::core::{
    interaction::SurfaceInteraction,
    ray::Ray,
    sampler::Sampler,
    scene::Scene,
    shape::Shape,
    spectrum::Spectrum,
    view::View
};

#[derive(Copy, Clone, Debug)]
pub struct RenderSettings {
    pub single_thread: bool
}

pub struct DirectLightingIntegrator {}

impl Default for DirectLightingIntegrator {
    fn default() -> Self {
        Self{}
    }
}

impl DirectLightingIntegrator {
    fn brdf_lambert(&self, spectrum: Spectrum) -> Spectrum
    {
        return spectrum / funty::Floating::PI;
    }

    fn li(&self, scene: &Scene, ray: &Ray, sampler: &mut Sampler) -> Spectrum {
        // TODO: Turn depth into a paramter
        const MAX_DEPTH: u32 = 10;

        let mut acc_color = Spectrum::ColorRGB(Vec3::from(0.0));
        let mut scatter_ray = ray.clone();

        for depth in 0..MAX_DEPTH {
            let mut isect = SurfaceInteraction::new();
            let hit = scene.intersect(&scatter_ray, &mut isect);
            if !hit {
                if depth == 0 {
                    acc_color = (scene.environment_light)(&ray);
                } else {
                    acc_color = acc_color * (scene.environment_light)(&ray);
                }
                break;
            }

            let mut material_color = Spectrum::ColorRGB(Vec3::from(1.0));

            if let Some(ref primitive) = isect.hit_primitive {
                if let Some(ref material) = primitive.material {
                    material_color = material.value().clone();
                    material.scatter(&mut scatter_ray, &mut material_color, &isect, sampler);

                    match &primitive.shape {
                        Shape::Mesh(mesh) => {
                            // TODO: Debug UV
                            let uv = isect.hit_uv;
                            // material_color = Spectrum::ColorRGB(Vec3::new(isect.hit_uv.x(), isect.hit_uv.y(), 0.0));
                            let x = (uv.x() * mesh.base_color_texture.width() as Float) as u32;
                            let y = (uv.y() * mesh.base_color_texture.height() as Float) as u32;
                            let base_color_texture = mesh.base_color_texture.clone().into_rgb8();
                            if base_color_texture.width() > 1 && base_color_texture.height() > 1 {
                                let rgb = base_color_texture.get_pixel(x, y);
                                material_color = Spectrum::ColorRGB(Vec3::new(rgb[0] as f32, rgb[1] as f32, rgb[2] as f32));
                            }
                        },
                        _ => {}
                    }
                }
            }

            // New ray
            scatter_ray.direction = scatter_ray.direction.normalize();
            scatter_ray.origin = isect.hit_point + scatter_ray.direction * 1e-3;

            let n_dot_l = Float::clamp(Vec3::dot(isect.hit_normal, scatter_ray.direction), 0., 1.);

            //acc_color = acc_color * self.brdf_lambert(material_color) * n_dot_l * 2.0 * PI;
            if depth == 0 {
                acc_color = material_color;
            } else {
                acc_color = acc_color * material_color;
            }
        }
        Spectrum::ColorRGB(acc_color.clamp(0., 1.))
    }

    pub fn render(&mut self, scene: Scene, view: View, render_setings: RenderSettings) -> Vec<Spectrum> {
        let samples_per_pixel = view.samples_per_pixel;

        let num_pixels = view.width as usize * view.height as usize;
        let mut pixels = vec![Spectrum::ColorRGB(Vec3::from(0.)); num_pixels];

        if render_setings.single_thread {
            let mut sampler = Sampler::default();

            // Single threaded version
            for y in (0..view.height).rev() {
                for x in 0..view.width {
                    let Spectrum::ColorRGB(mut total_spectrum) = Spectrum::ColorRGB(Vec3::from(0.));
                    for _ in 0..samples_per_pixel {
                        let uv: Vec2 = sampler.sample_from_pixel(Vec2 {0: x as Float, 1: y as Float}, view.width, view.height);
                        let mut ray = scene.persp_camera.get_ray(&uv, &mut sampler);
                        let li = self.li(&scene, &mut ray, &mut sampler);
                        match li {
                            Spectrum::ColorRGB(li) => total_spectrum = li + total_spectrum
                        }
                    }
                    total_spectrum /= samples_per_pixel as Float;
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
                    let mut sampler = Sampler::default();
                    let uv: Vec2 = sampler.sample_from_pixel(Vec2 {0: x as Float, 1: y as Float}, view.width, view.height);

                    let ray = scene.persp_camera.get_ray(&uv, &mut sampler);
                    let Spectrum::ColorRGB(color) = self.li(&scene.clone(), &ray.clone(), &mut sampler);
                    color
                }).sum::<Vec3>();

                total_spectrum /= samples_per_pixel as Float;
                // Gamma correction
                total_spectrum = Vec3::sqrt(total_spectrum);
                *pixel = Spectrum::ColorRGB(total_spectrum);
            });
            // Uncomment to test parallel thread on samples level.
            // for i in 0..num_pixels {
            //     let x: u32 = i as u32 % view.width;
            //     let y: u32 = i as u32 / view.width;
            //     let mut total_spectrum = (0..samples_per_pixel).into_par_iter().
            //     map(|_sample| {
            //         let mut sampler = Sampler::new();
            //         let uv: Vec2 = sampler.sample_from_pixel(Vec2 {0: x as Float, 1: y as Float}, view.width, view.height);

            //         let ray = scene.persp_camera.get_ray(&uv, &mut sampler);
            //         let Spectrum::ColorRGB(color) = self.li(&scene.clone(), &ray.clone(), &mut sampler);
            //         color
            //     }).sum::<Vec3>();
            //     total_spectrum /= samples_per_pixel as Float;
            //     // Gamma correction
            //     total_spectrum = Vec3::sqrt(total_spectrum);
            //     pixels[i] = Spectrum::ColorRGB(total_spectrum);
            // }

        }
        return pixels;
    }
}