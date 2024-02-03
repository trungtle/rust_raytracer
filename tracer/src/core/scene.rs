use std::path::Path;

use math::Vec3;

use crate::cameras::perspective::PerspectiveCamera;
use crate::core::primitive::Primitive;
use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;
use crate::core::spectrum::Spectrum;
use crate::core::transform::Transform;

use super::transform;

#[derive(Clone, PartialEq, Debug)]
pub struct Scene {
    pub primitives: Vec<Primitive>,
    pub environment_light: fn(&Ray) -> Spectrum,
    pub persp_camera: PerspectiveCamera,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            primitives: Vec::default(),
            environment_light: |_| Spectrum::ColorRGB(Vec3::from(0.)),
            persp_camera: PerspectiveCamera::default()
        }
    }
}

impl Scene {
    fn parse_gltf_mesh(self, mesh: &gltf::Mesh, xform: Transform) {

    }

    fn parse_gltf_node(self, node: &gltf::Node, parent_xform: Transform) {
        let xform = parent_xform * node.transform();
        if let Some(mesh) = node.mesh() {

        }

        for child_node in node.children() {
            self.parse_gltf_node(child_node, )
        }
    }
}

impl Scene {
    pub fn from_gltf<P>(path: P) -> Self
    where P: AsRef<Path>
    {
        let (doc, buffers, images) = gltf::import(path).unwrap();

        let mut primitives = Vec::default();

        Self {
            primitives,
            environment_light: |_| Spectrum::ColorRGB(Vec3::from(0.)),
            persp_camera: PerspectiveCamera::default()
        }
    }

    pub fn add(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }

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