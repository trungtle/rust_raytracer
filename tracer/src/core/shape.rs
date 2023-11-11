use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;
use crate::shapes::mesh::Mesh;
use crate::shapes::sphere::Sphere;
use crate::shapes::triangle::Triangle;

use super::Transform;

#[derive(Clone)]
pub enum Shape {
    Mesh(Mesh),
    Sphere(Sphere),
    Triangle(Triangle)
}

impl Shape {
    pub fn apply_transform(&mut self, transform: &Transform) {
        match self {
            Shape::Mesh(shape) => {
                for position in shape.positions.iter_mut() {
                    *position = transform.matrix * (*position);
                }
            }
            Shape::Sphere(shape) => {
                shape.center = transform.get_position() + shape.center;
            }
            Shape::Triangle(shape) => {
                shape.v0 = transform.matrix * shape.v0;
                shape.v1 = transform.matrix * shape.v1;
                shape.v2 = transform.matrix * shape.v2;
            }
        }
    }

    pub fn intersect(&self, ray: &Ray, isect: &mut SurfaceInteraction) -> bool {
        match self {
            Shape::Mesh(shape) => shape.intersect(ray, isect),
            Shape::Sphere(shape) => shape.intersect(ray, isect),
            Shape::Triangle(shape) => shape.intersect(ray, isect),
        }
    }
}