use std::sync::Arc;

use crate::core::geometry::Bounds3;
use crate::core::interaction::SurfaceInteraction;
use crate::core::material::Material;
use crate::core::ray::Ray;
use crate::core::shape::Shape;

#[derive(Clone)]
pub enum Primitive {
    Shape(Box<ShapePrimitive>)
}

impl Primitive {
    // pub fn world_bound(&self) -> Bounds3 {
    //     match self {
    //         Primitive::Shape(primitive) => primitive.world_bound(),
    //     }
    // }

    pub fn intersect(&self, ray: &Ray, isect: &mut SurfaceInteraction) -> bool {
        match self {
            Primitive::Shape(primitive) => primitive.shape.intersect(ray, isect)
        }
    }
}

#[derive(Clone)]
pub struct ShapePrimitive {
    pub shape: Shape,
    pub material: Option<Box<Material>>
}

impl ShapePrimitive {
    pub fn new(
        shape: Shape,
        material: Option<Box<Material>>
    ) -> Self {
        ShapePrimitive {
            shape,
            material
        }
    }
}