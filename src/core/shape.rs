use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;
use crate::shapes::sphere::Sphere;
use crate::shapes::triangle::Triangle;

#[derive(Clone)]
pub enum Shape {
    Sphere(Sphere),
    Triangle(Triangle)
}

impl Shape {
    pub fn intersect(&self, ray: &Ray, isect: &mut SurfaceInteraction) -> bool {
        match self {
            Shape::Sphere(shape) => shape.intersect(ray, isect),
            Shape::Triangle(shape) => shape.intersect(ray, isect),
        }
    }
}