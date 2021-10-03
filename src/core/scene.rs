use crate::cameras::PerspectiveCamera;
use crate::core::interaction::SurfaceInteraction;
use crate::materials::MatteMaterial;
use crate::math::{Vec2, Vec3};
use crate::ray::Ray;

pub trait Hitable {
    fn hit(&self, ray: &Ray) -> SurfaceInteraction;
}

pub trait Shape {
    fn normal_at(&self, _point: &Vec3) -> Vec3 { Vec3::from(0.) }
    fn uv_at(&self, _point: &Vec3) -> Vec2 { Vec2::from(0.) }
}

pub trait SceneObject: Hitable + Shape + Send + Sync {}
impl<T: Hitable + Shape + Send + Sync> SceneObject for T {}

pub struct Scene {
    drawables: Vec<Box<dyn SceneObject>>,
    pub persp_camera: PerspectiveCamera,
}

impl Scene {
    pub fn new(persp_camera: PerspectiveCamera) -> Self {
        Self {
            drawables: Vec::new(),
            persp_camera
        }
    }

    pub fn add(&mut self, drawable: Box<dyn SceneObject>) {
        self.drawables.push(drawable);
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