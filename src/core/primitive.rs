use std::sync::Arc;

use crate::geometry::Bounds3;

pub enum Primitive {
    Shape(Box<ShapePrimitive>)
}

impl Primitive {
    pub fn world_bound(&self) -> Bounds3 {
        match self {
            Primitive::Shape(primitive) => primitive.world_bound(),
        }
    }
}

#[derive(Clone)]
pub struct ShapePrimitive {
    pub 
}