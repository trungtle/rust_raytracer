use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::core::{
    Hitable,
    Scene,
    Spectrum,
    View
};
use crate::integrators::Integrator;
use crate::materials::{
    Pdf,
    UniformPdf
};
use crate::math:: {
    Sampler,
    Vec2,
    Vec3
};
use crate::ray::Ray;
use rayon::prelude::*;

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
        const MAX_DEPTH: u32 = 10;

        let Spectrum::ColorRGB(mut acc_color) = Spectrum::ColorRGB(Vec3::from(0.));

        let mut scatter_ray = ray;
        
        for depth in 0..MAX_DEPTH {
            let hit = self.scene.hit(&scatter_ray);
            if hit.t > 0. {
                let Spectrum::ColorRGB(bounce_color) = Spectrum::ColorRGB(Vec3::new(0.5, 0.5, 0.5));
                let uniform_pdf = UniformPdf::new(&hit.hit_normal);
                
                let new_direction = uniform_pdf.sample_wi();
                scatter_ray.origin = hit.hit_point;
                scatter_ray.direction = new_direction.normalize();

                let scattering_pdf = f64::abs(Vec3::dot(hit.hit_normal, scatter_ray.direction));

                if depth == 0 {
                    acc_color = bounce_color;
                }
                else {
                    acc_color = scattering_pdf * acc_color * bounce_color;
                }
                
            } else {
                let t = 0.5 * ray.direction.y + 1.0;
                let sky_color = (1. - t) * Vec3::new(1.,1.,1.) + t * Vec3::new(0.5, 0.7, 1.);
                if depth == 0 {
                    acc_color = sky_color;
                } else {
                    acc_color = acc_color * sky_color;
                }
                break;
            }
        }
        Spectrum::ColorRGB(acc_color.clamp(0., 1.))
    }
}

impl Integrator for DirectLightingIntegrator {
    fn render(&mut self, view: &View) -> Spectrum {

        let samples_per_pixel = 100;
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

        (0..view.height*view.width).into_par_iter().for_each(|i| {
            let x: u32 = i % view.width;
            let y: u32 = i / view.width;
            let Spectrum::ColorRGB(mut total_spectrum) = Spectrum::ColorRGB(Vec3::from(0.));
            (0..samples_per_pixel).into_par_iter().for_each(|_sample| {
                let uv: Vec2 = Sampler::sample_from_pixel(Vec2 {x: x as f64, y: y as f64}, view.width, view.height);
                let ray = self.scene.persp_camera.get_ray(&uv);
                match self.li(&ray) {
                    Spectrum::ColorRGB(li) => total_spectrum = li + total_spectrum
                } 
                
            });
            total_spectrum /= samples_per_pixel as f64;
            self.scene.persp_camera.write_to_film(x, y, Spectrum::ColorRGB(total_spectrum));
        });

        // Parallel version
        // let mut handles = vec![];
        // let shared_scene = Arc::new(self.scene);

        // for thread_index in 0..(view.width * view.height) {
        //     let shared_scene = Arc::clone(&shared_scene);
        //     let handle = thread::spawn(move || {
                
        //         let Spectrum::ColorRGB(mut total_spectrum) = Spectrum::ColorRGB(Vec3::from(0.));
        //         for _ in 0..samples_per_pixel {
        //             let x = thread_index % view.width;
        //             let y = thread_index / view.width;
        //             let uv: Vec2 = Sampler::sample_from_pixel(Vec2 {x: x as f64, y: y as f64}, view.width, view.height);
                    
        //             let mut ray = shared_scene.persp_camera.get_ray(&uv);
        //             let li = self.li(&mut ray);
        //             match li {
        //                 Spectrum::ColorRGB(li) => total_spectrum = li + total_spectrum
        //             }    
        //         }
        //     });
        //     handles.push(handle);
        // }
        
        self.scene.persp_camera.write_film_to_file();
        Spectrum::ColorRGB(Vec3::from(0.))
    }
}