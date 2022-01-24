use gltf;
use log::{info, debug, trace, warn};

use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;
use crate::loaders::gltf_loader::GData;
use crate::math::vectors::{Vec2, Vec3};
use crate::shapes::triangle::Triangle;

#[derive(Clone, Debug)]
pub struct Mesh {
    pub positions: Vec<Vec3>,
    pub indices: Vec<u32>
}

impl Mesh {
    pub fn new(positions: Vec<Vec3>, indices: Vec<u32>) -> Self {
        Self {
            positions, indices
        }
    }

    pub fn from_gltf(g_primitive: &gltf::Primitive, g_data: &GData) -> Self {
        let mut positions: Vec<Vec3> = vec![];
        let mut indices: Vec<u32> = vec![];

        // Positions
        let reader = g_primitive.reader(|buffer| Some(&g_data.buffers[buffer.index()]));
        if let Some(iter) = reader.read_positions() {
            for vertex_position in iter {
                let x = vertex_position[0] as f64;
                let y = vertex_position[1] as f64;
                let z = vertex_position[2] as f64;
                positions.push(Vec3::new(x, y, z));
            }
        }
        
        // Indices
        if let Some(iter) = reader.read_indices() {
            for index in iter.into_u32() {
                indices.push(index);
            }
        }

        println!("Positions size {}", positions.len());
        println!("Positions {:?}", positions);
        println!("Indices {:?}", indices);
        Self {
            positions,
            indices
        }
    }

    pub fn intersect(&self, ray: &Ray, isect: &mut SurfaceInteraction) -> bool {

        let mut hit = false;
        let mut nearest_t = 9999.9;
        let mut nearest_isect = SurfaceInteraction::new();
        let max_index = self.indices.len()-1;

        for i in (0..max_index).step_by(3) {
            let triangle = Triangle::new(
                self.positions[self.indices[i] as usize],
                self.positions[self.indices[i+1] as usize],
                self.positions[self.indices[i+2] as usize],
            );

            let tri_hit = triangle.intersect(ray, isect);
            if tri_hit && isect.t < nearest_t {
                hit = true;
                nearest_t = isect.t;

                nearest_isect.t = isect.t;
                nearest_isect.hit_point = isect.hit_point;
                nearest_isect.hit_normal = isect.hit_normal;
                nearest_isect.hit_uv = isect.hit_uv;
            }
        }
        
        isect.t = nearest_isect.t;
        isect.hit_point = nearest_isect.hit_point;
        isect.hit_normal = nearest_isect.hit_normal;
        isect.hit_uv = nearest_isect.hit_uv;
        return hit;
    }
}
