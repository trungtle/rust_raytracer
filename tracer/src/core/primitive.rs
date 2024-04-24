use std::sync::Arc;

use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;
use crate::core::shape::Shape;
use crate::core::Transform;
use crate::materials::Material;

#[derive(Clone, PartialEq, Debug)]
pub struct Primitive {
    pub shape: Shape,
    pub material: Option<Arc<dyn Material>>,
    transform: Transform,
}

impl Primitive {
    pub fn new(shape: Shape, material: Option<Arc<dyn Material>>) -> Self {
        Primitive {
            shape,
            material,
            transform: Transform::default(),
        }
    }

    // pub fn from_pbrt4(shape: pbrt4::ShapeEntity) -> Self {
    //     Self {
    //         shape: Mesh()
    //         transform: shape.transform
    //     }
    // }

    pub fn transform(&self) -> Transform {
        self.transform.clone()
    }

    pub fn apply_transform(&mut self, new_transform: Transform) {
        self.transform = new_transform;
        self.shape.apply_transform(&self.transform);
    }

    pub fn intersect(&self, ray: &Ray, isect: &mut SurfaceInteraction) -> bool {
        self.shape.intersect(ray, isect)
    }
}
