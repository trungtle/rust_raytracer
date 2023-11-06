use std::sync::Arc;

use crate::core::geometry::Bounds3;
use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;
use crate::core::shape::Shape;
use crate::materials::Material;

#[derive(Clone)]
pub struct Primitive {
    pub shape: Shape,
    pub material: Option<Arc<dyn Material>>,

}

impl Primitive {
    pub fn new(shape: Shape, material: Option<Arc<dyn Material>>) -> Self {
        Primitive { shape, material }
    }

    pub fn intersect(&self, ray: &Ray, isect: &mut SurfaceInteraction) -> bool {
        self.shape.intersect(ray, isect)
    }
}
