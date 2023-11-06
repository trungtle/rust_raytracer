use crate::cameras::perspective::PerspectiveCamera;
use crate::core::primitive::Primitive;
use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;
use crate::core::spectrum::Spectrum;
use crate::math::vectors::Vec3;

pub struct Scene {
    pub primitives: Vec<Primitive>,
    pub environment_light: fn(&Ray) -> Spectrum,
    pub persp_camera: PerspectiveCamera,
}

impl Scene {
    pub fn new(persp_camera: PerspectiveCamera) -> Self {
        Self {
            primitives: Vec::new(),
            environment_light: |ray| Spectrum::ColorRGB(Vec3::from(0.)),
            persp_camera
        }
    }

    pub fn add(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }
}

impl Scene {
    pub fn intersect(&self, ray: &Ray, closest_isect: &mut SurfaceInteraction) -> bool {
        const MAX_T: f64 = 99999.;
        let mut closest_t = MAX_T;
        for primitive in self.primitives.iter() {
            let mut isect = SurfaceInteraction::new();
            let hit = primitive.intersect(&ray, &mut isect);
            if hit && isect.t < closest_t{
                closest_t = isect.t;
                closest_isect.hit_normal = isect.hit_normal;
                closest_isect.hit_point = isect.hit_point;
                closest_isect.hit_uv = isect.hit_uv;
                closest_isect.hit_primitive = Some(primitive.clone());
            }
        }
        if closest_t < MAX_T && closest_t > 1e-5 {
            return true;
        } else {
            return false;
        }
    }
}