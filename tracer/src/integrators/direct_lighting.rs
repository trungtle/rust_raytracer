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

use crate::materials::{
    pdf::Pdf,
    pdf::UniformPdf
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

    fn environment_light(&self, ray: &Ray) -> Spectrum {
        let t = 0.5 * ray.direction.y + 1.0;
        // let background_color = (1. - t) * Vec3::new(1.,1.,1.) + t * Vec3::new(0.5, 0.7, 1.);
        // Background all white as furnace test

        let uniform_environment = Spectrum::ColorRGB(Vec3 { x: 0.5, y: 0.5, z: 0.5 });
        return uniform_environment;
    }

    fn brdf_lambert(&self, spectrum: Spectrum) -> Spectrum
    {
        return spectrum / PI;
    }

    fn li(&self, ray: &Ray) -> Spectrum {
        const MAX_DEPTH: u32 = 15;

        let mut acc_color = Spectrum::ColorRGB(Vec3::from(0.));
        let mut mask = Spectrum::ColorRGB(Vec3::from(1.));
        let mut scatter_ray = ray.clone();

        for _depth in 0..MAX_DEPTH {
            let mut isect = SurfaceInteraction::new();
            let hit = self.scene.intersect(&scatter_ray, &mut isect);
            if !hit {
                acc_color = acc_color + mask * self.environment_light(ray);
                break;
            }

            let mut material_color = Spectrum::ColorRGB(Vec3::new(0.5,0.5,0.5));

            match isect.hit_primitive {
                Some(primitive) => {
                    match *primitive {
                        Primitive::Shape(primitive) => {
                            match primitive.material {
                                Some(material) => {
                                    match *material {
                                        Material::Constant(material) => {
                                            material_color = material.get_value().clone();
                                        },
                                        Material::Matte(_material) => {

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
            let uniform_pdf = UniformPdf::new(&isect.hit_normal);
            scatter_ray.origin = isect.hit_point;
            scatter_ray.direction = uniform_pdf.sample_wi().normalize();

            let NdotL = f64::clamp(Vec3::dot(isect.hit_normal, scatter_ray.direction), 0., 1.);

            mask = mask * self.brdf_lambert(material_color) * NdotL * 2.0 * PI;

            if Spectrum::dot(mask, mask) < 1e-3
            {
                break;
            }
        }
        Spectrum::ColorRGB(acc_color.clamp(0., 1.))
    }
}

impl DirectLightingIntegrator {
    pub fn render(&mut self, view: &View) {
        let samples_per_pixel = 10;

        // Single threaded version
        // for y in (0..view.height).rev() {
        //     for x in 0..view.width {
        //         let Spectrum::ColorRGB(mut total_spectrum) = Spectrum::ColorRGB(Vec3::from(0.));
        //         for _ in 0..samples_per_pixel {
        //             let uv: Vec2 = Sampler::sample_from_pixel(Vec2 {x: x as f64, y: y as f64}, view.width, view.height);
        //             let mut ray = self.scene.persp_camera.get_ray(&uv);
        //             let li = self.li(&mut ray);
        //             match li {
        //                 Spectrum::ColorRGB(li) => total_spectrum = li + total_spectrum
        //             }
        //         }
        //         total_spectrum /= samples_per_pixel as f64;
        //         self.scene.persp_camera.write_to_film(x, y, Spectrum::ColorRGB(total_spectrum));
        //     }
        // }

        // Parallel version
        let num_pixels = view.width as usize * view.height as usize;
        let mut pixels = vec![Spectrum::ColorRGB(Vec3::from(0.)); num_pixels];

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
            total_spectrum = Vec3::sqrt(total_spectrum);
            *pixel = Spectrum::ColorRGB(total_spectrum);
        });

        self.scene.persp_camera.set_pixels(pixels);
        self.scene.persp_camera.write_film_to_file();
    }
}