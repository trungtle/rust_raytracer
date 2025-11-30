use std::f32::consts::PI;

use log::info;
use rayon::prelude::*;

use math::{Float, Vec2, Vec3};

use crate::core::{
    interaction::SurfaceInteraction, ray::Ray, sampler::Sampler, scene::Scene, shape::Shape,
    spectrum::Spectrum, view::View,
};

pub const SCREEN_WIDTH: u32 = 400;
pub const SCREEN_HEIGHT: u32 = 400;
pub const SAMPLES_PER_PIXEL: u8 = 5;

#[derive(Copy, Clone, Debug)]
pub struct RenderSettings {
    pub single_thread: bool,
    pub write_to_file: bool,
    pub skycolor_tint: [f32; 3],
    pub current_depth: i32,
    pub sample_per_pixel: u8,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            single_thread: false,
            write_to_file: false,
            skycolor_tint: [1.0, 1.0, 1.0],
            current_depth: 0,
            sample_per_pixel: SAMPLES_PER_PIXEL
        }
    }
}


// A width x height framebuffer to track the progressive render state.
// Stores color, ray, depth, and termination state of each fragment.
#[derive(Clone, Debug)]
pub struct FrameBuffer {
    pub depths: Vec<u32>,
    pub spectrums: Vec<Spectrum>,
    pub rays: Vec<Ray>,
    pub terminated: Vec<bool>,
    pub current_sample: u8,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let num_fragments = (width * height) as usize;
        Self {
            depths: vec![0; num_fragments],
            spectrums: vec![Spectrum::default(); num_fragments],
            rays: vec![Ray::default(); num_fragments],
            terminated: vec![false; num_fragments],
            current_sample: 0
        }
    }
}

#[derive(Clone, Debug)]
pub struct Fragment {
    pub depth: u32,
    pub acc_spectrum: Spectrum,
    pub ray: Ray,
    pub terminate: bool,
}

impl Default for Fragment {
    fn default() -> Self {
        Self {
            depth: 0,
            acc_spectrum: Spectrum::ColorRGB(Vec3::from(0.0)),
            ray: Ray::default(),
            terminate: false,
        }
    }
}

pub struct DirectLightingIntegrator {}

impl DirectLightingIntegrator {
    fn brdf_lambert(spectrum: Spectrum) -> Spectrum {
        return spectrum / funty::Floating::PI;
    }
    
    // Raytrace one fragment (pixel)
    fn li(fragment: &Fragment, scene: &Scene, settings: &RenderSettings) -> Fragment {
        // TODO: Turn depth into a paramter
        const MAX_DEPTH: u32 = 100;

        let mut new_fragment = Fragment::default();
        new_fragment.ray = fragment.ray;
        //new_fragment.acc_spectrum = Spectrum::ColorRGB(Vec3::from(0.0));
        for depth in 0..MAX_DEPTH {
            new_fragment.depth = depth;
            new_fragment = DirectLightingIntegrator::li_one_bounce(&new_fragment, &scene.clone(), &settings);
            if new_fragment.terminate {
                break;
            }
        }

        // Finished rendering past depth
        new_fragment.terminate = true;
        return new_fragment;
    }

    // Ray trace one bounce
    fn li_one_bounce( 
        current_fragment: &Fragment,
        scene: &Scene,
        render_setings: &RenderSettings
    ) -> Fragment {
            let mut new_fragment = current_fragment.clone();
            let mut isect = SurfaceInteraction::new();
            let hit = scene.intersect(&current_fragment.ray, &mut isect);

            // Failed to hit anything. Return
            if !hit {
                new_fragment.terminate = true;

                // Blend with the environment light
                let environment_spectrum = (scene.environment_light)(&current_fragment.ray);// * Spectrum::ColorRGB(Vec3::from(&render_setings.skycolor_tint));
                if current_fragment.depth == 0 {
                    new_fragment.acc_spectrum = environment_spectrum;
                } else {
                    new_fragment.acc_spectrum = current_fragment.acc_spectrum.clone() * environment_spectrum;
                }
                return new_fragment;
            }

            // Hit something. Determines color now.
            let mut material_color = Spectrum::ColorRGB(Vec3::from(1.0));

            if let Some(ref primitive) = isect.hit_primitive {
                if let Some(ref material) = primitive.material {
                    material_color = material.value().clone();
                    let scatter_result = material.scatter(&current_fragment.ray, &mut material_color, &isect);
                    // New ray
                    new_fragment.ray.direction = scatter_result.ray.direction.normalize();
                    new_fragment.ray.origin = isect.hit_point + new_fragment.ray.direction * 1e-3;

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
                                material_color = Spectrum::ColorRGB(Vec3::new(
                                    rgb[0] as f32,
                                    rgb[1] as f32,
                                    rgb[2] as f32,
                                ));
                            }
                        }
                        _ => {}
                    }
                }
            }

            // let n_dot_l = Float::clamp(Vec3::dot(isect.hit_normal, new_fragment.ray.direction), 0., 1.);
            // new_fragment.acc_spectrum = DirectLightingIntegrator::brdf_lambert(material_color) * n_dot_l * 2.0 * PI;
            if new_fragment.depth == 0 {
                new_fragment.acc_spectrum = material_color;
            } else {
                new_fragment.acc_spectrum = current_fragment.acc_spectrum * material_color;
            }
            return new_fragment;
    }

    fn render_single_thread(
        scene: &Scene,
        view: &View,
        framebuffer: &FrameBuffer,
        render_settings: &RenderSettings
    ) -> FrameBuffer {
        let mut new_frame = FrameBuffer::new(view.width, view.height);
        new_frame.current_sample = framebuffer.current_sample;

        // Single threaded version
        for y in 0..view.height {
            for x in 0..view.width {

                // Initialize fragment
                let frag_index = (x + (view.height - y - 1) * view.width) as usize;
                let mut fragment = Fragment {
                    depth: framebuffer.depths[frag_index],
                    acc_spectrum: framebuffer.spectrums[frag_index],
                    ray: framebuffer.rays[frag_index],
                    terminate: framebuffer.terminated[frag_index]

                };
                
                let Spectrum::ColorRGB(mut acc_spectrum) = fragment.acc_spectrum;
                // for _ in 0..samples_per_pixel {
                    let uv: Vec2 = Sampler::sample_from_pixel(
                        Vec2 {
                            0: x as Float,
                            1: y as Float,
                        },
                        view.width,
                        view.height,
                    );

                    fragment.ray = scene.persp_camera.get_ray(&uv);
                    let new_fragment = DirectLightingIntegrator::li(&fragment, &scene, &render_settings);

                    let Spectrum::ColorRGB(new_spectrum) = new_fragment.acc_spectrum;

                    // Copy the state of this fragment to new framebuffer,
                    // except for the color since we still need to average over all the samples.
                    // new_frame.depths[frag_index] = new_fragment.depth;
                    // new_frame.rays[frag_index] = new_fragment.ray;
                    new_frame.terminated[frag_index] = new_fragment.terminate;
                // }
                acc_spectrum = acc_spectrum * (framebuffer.current_sample - 1) as Float;
                acc_spectrum = (acc_spectrum + new_spectrum) / (framebuffer.current_sample as Float);                    
                acc_spectrum = acc_spectrum.clamp(0., 1.);
                new_frame.spectrums[frag_index] = Spectrum::ColorRGB(acc_spectrum);
            }
        }
        return new_frame;
    }

    pub fn render_parallel(
        scene: &Scene,
        view: &View,
        framebuffer: &FrameBuffer,
        render_setings: &RenderSettings
    ) -> FrameBuffer {
        let mut new_frame = FrameBuffer::new(view.width, view.height);
        new_frame.current_sample = framebuffer.current_sample;

        let mut total_spectrums = framebuffer.spectrums.clone();
        total_spectrums.par_iter_mut().enumerate().for_each(|(i, acc_spectrum)| {
            // For each pixel
            let x: u32 = i as u32 % view.width;
            let y: u32 = i as u32 / view.width;

            // Initialize fragment
            let frag_index = (x + (view.height - y - 1) * view.width) as usize;
            let mut fragment = Fragment {
                depth: framebuffer.depths[frag_index],
                acc_spectrum: *acc_spectrum,
                ray: framebuffer.rays[frag_index],
                terminate: framebuffer.terminated[frag_index]

            };
                            
            // Accumulate the total spectrum over all the samples per pixel
            // let mut total_spectrum = (0..samples_per_pixel)
            //     .into_par_iter()
            //     .map(|_sample| {
                    let uv: Vec2 = Sampler::sample_from_pixel(
                        Vec2 {
                            0: x as Float,
                            1: y as Float,
                        },
                        view.width,
                        view.height,
                    );

                    fragment.ray = scene.persp_camera.get_ray(&uv);
                    let new_fragment = DirectLightingIntegrator::li(&fragment, &scene, &render_setings);

                    let Spectrum::ColorRGB(new_spectrum) = new_fragment.acc_spectrum;

                    // Copy the state of this fragment to new framebuffer,
                    // except for the color since we still need to average over all the samples.
                    // new_frame.depths[frag_index] = new_fragment.depth;
                    // new_frame.rays[frag_index] = new_fragment.ray;
                    // new_frame.terminated[frag_index] = new_fragment.terminate;

                    // Return the accumulated color to sum it up
                    // let Spectrum::ColorRGB(color) = new_fragment.acc_spectrum;
                    // color
                // })
                // .sum::<Vec3>();

            // total_spectrum /= samples_per_pixel as Float;
            *acc_spectrum = *acc_spectrum * (framebuffer.current_sample - 1) as Float;
            *acc_spectrum = (*acc_spectrum + new_spectrum) / (framebuffer.current_sample as Float);
        });

        new_frame.spectrums = total_spectrums;
        return new_frame;   
    }

    pub fn render(
        scene: &Scene,
        view: &View,
        framebuffer: &FrameBuffer,
        render_setings: &RenderSettings
    ) -> FrameBuffer {
        if render_setings.single_thread {
            DirectLightingIntegrator::render_single_thread(scene, view, framebuffer, render_setings)
        } else {
            // Parallel version
            DirectLightingIntegrator::render_parallel(scene, view, framebuffer, render_setings)
        }
    }
}
