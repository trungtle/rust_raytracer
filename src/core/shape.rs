use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;
use crate::shapes::mesh::Mesh;
use crate::shapes::sphere::Sphere;
use crate::shapes::triangle::Triangle;

#[derive(Clone)]
pub enum Shape {
    Mesh(Mesh),
    Sphere(Sphere),
    Triangle(Triangle)
}

impl Shape {
    pub fn intersect(&self, ray: &Ray, isect: &mut SurfaceInteraction) -> bool {
        match self {
            Shape::Mesh(shape) => shape.intersect(ray, isect),
            Shape::Sphere(shape) => shape.intersect(ray, isect),
            Shape::Triangle(shape) => shape.intersect(ray, isect),
        }
    }
}