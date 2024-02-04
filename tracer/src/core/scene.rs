use std::borrow::BorrowMut;
use std::path::Path;
use std::sync::Arc;

use math::{Float, Quaternion, Vec3};

use crate::cameras::perspective::PerspectiveCamera;
use crate::core::primitive::Primitive;
use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;
use crate::core::spectrum::Spectrum;
use crate::core::transform::Transform;
use crate::core::shape::Shape;
use crate::loaders::gltf_loader::GData;
use crate::materials::LambertMaterial;
use crate::shapes::mesh::Mesh;

use log::info;

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
    fn parse_gltf<P>(path: P) -> Self 
        where P: AsRef<Path> {
        let (doc, buffers, images) = gltf::import(path).unwrap();
        let data = GData { doc , buffers, images };

        let mut scene = Scene::default();

        for node in data.doc.nodes() {
            Scene::parse_gltf_node(&mut scene, &data, &node, Transform::default());
        }

        for image in data.images {

        }

        return scene;
    }

    fn parse_gltf_node(scene: &mut Scene, data: &GData, node: &gltf::Node, parent_xform: Transform) {
        let xform = parent_xform * Transform::from(&node.transform());

        for child_node in node.children() {
            Scene::parse_gltf_node(scene, data, &child_node, xform);
        }

        if let Some(mesh) = node.mesh() {
            info!("Node: {:?} - Mesh: {:?} - Transform: {}", node.name(), mesh.name(), xform);
            Scene::parse_gltf_mesh(scene, data, &mesh, xform);
        }
        else if let Some(_) = node.camera() {
           scene.persp_camera.set_position(&xform.get_position());
        }
    }

    fn parse_gltf_mesh(scene: &mut Scene, data: &GData, mesh: &gltf::Mesh, xform: Transform) {
        for primitive in mesh.primitives() {
            let mesh = Mesh::from_gltf(&primitive, &data);
            let mut primitive = Primitive::new(Shape::Mesh(mesh),
                Option::Some(
                    Arc::new(LambertMaterial::new(Spectrum::ColorRGB(Vec3::new(0.5, 0.5, 0.5))))));
            primitive.apply_transform(xform);
            scene.add(primitive);
        }
    }
}

impl<P> From<P> for Scene
    where P: AsRef<Path> {
    fn from(path: P) -> Self {
        if let Some(extension) = path.as_ref().extension() {
            if extension == "gltf" || extension == "glb" {
                return Scene::parse_gltf(path);
            }
        }

        Scene::default()
    }
}

impl Scene {
    pub fn add(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }

    pub fn intersect(&self, ray: &Ray, closest_isect: &mut SurfaceInteraction) -> bool {
        const MAX_T: Float = funty::Floating::MAX;
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
        if closest_t < MAX_T && closest_t > funty::Floating::EPSILON {
            return true;
        } else {
            return false;
        }
    }
}