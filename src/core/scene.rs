use crate::cameras::perspective::PerspectiveCamera;
use crate::core::primitive::Primitive;
use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;

pub struct Scene {
    primitives: Vec<Primitive>,
    pub persp_camera: PerspectiveCamera,
}

impl Scene {
    pub fn new(persp_camera: PerspectiveCamera) -> Self {
        Self {
            primitives: Vec::new(),
            persp_camera
        }
    }

    pub fn add(&mut self, primitives: Primitive) {
        self.primitives.push(primitives);
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
                closest_isect.hit_primitive = Some(Box::new(primitive.clone())); 
            }
        }
        if closest_t < MAX_T {
            return true;
        } else {
            return false;
        }
    }
}