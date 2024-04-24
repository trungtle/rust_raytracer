use std::primitive;

use crate::core::interaction::SurfaceInteraction;
use crate::core::ray::Ray;
use crate::loaders::gltf_loader::GData;
use crate::shapes::triangle::Triangle;
use gltf;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use log::info;
use math::{Float, Vec2, Vec3};

#[derive(Clone, PartialEq, Debug)]
pub struct Mesh {
    pub indices: Vec<u32>,
    pub positions: Vec<Vec3>,
    pub uv: Vec<Vec2>,
    pub base_color_texture: image::DynamicImage,
}

fn load_image_from_path(
    path: &std::path::Path,
) -> Result<eframe::egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(eframe::epaint::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

impl Mesh {
    pub fn new(positions: Vec<Vec3>, indices: Vec<u32>) -> Self {
        Self {
            indices: indices,
            positions: positions,
            uv: Vec::new(),
            base_color_texture: image::DynamicImage::new_rgb8(1, 1),
        }
    }

    // pub fn from_pbrt4(shape: pbrt4::ShapeEntity) -> Self {
    //     let mut positions: Vec<Vec3> = vec![];
    //     let mut indices: Vec<u32> = vec![];

    //     shape.params
    // }

    pub fn from_ply(filepath: &std::path::Path) {
        info!("Parse ply model file path {:?}", filepath.to_str());
        let ply_model = mesh_loader::parse_ply(filepath);
        match ply_model {
            Ok(mut ply_model) => {
                for entry in ply_model.payload.entries().into_iter() {
                    println!("K: {}, V: {:?}", entry.key(), entry.get().first());
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    pub fn from_gltf(primitive: &gltf::Primitive, data: &GData) -> Self {
        use gltf::mesh::util::ReadTexCoords::{F32, U16, U8};

        let mut positions: Vec<Vec3> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut uv: Vec<Vec2> = vec![];

        let reader = primitive.reader(|buffer| Some(&data.buffers[buffer.index()]));

        // Indices
        if let Some(iter) = reader.read_indices() {
            for index in iter.into_u32() {
                indices.push(index);
            }
        }

        // Positions
        if let Some(iter) = reader.read_positions() {
            for vertex_position in iter {
                positions.push(Vec3::from(&vertex_position));
            }
        }

        // UVs
        // TODO: Need to read from multiple UVs sets
        if let Some(read_tex_coords) = reader.read_tex_coords(0) {
            match read_tex_coords {
                // NOTE: Can we just convert from U8, U16 into float like this?
                U8(iter) => {
                    for _uv in iter {
                        let u = _uv[0] as Float;
                        let v = _uv[1] as Float;
                        uv.push(Vec2::new(&[u, v]));
                    }
                }
                U16(iter) => {
                    for _uv in iter {
                        let u = _uv[0] as Float;
                        let v = _uv[1] as Float;
                        uv.push(Vec2::new(&[u, v]));
                    }
                }
                F32(iter) => {
                    for _uv in iter {
                        uv.push(Vec2::new(&_uv));
                    }
                }
            }
        }

        // Textures
        let mut base_color_texture = image::DynamicImage::new_rgb8(1, 1);
        if let Some(texture) = primitive
            .material()
            .pbr_metallic_roughness()
            .base_color_texture()
        {
            // TODO: Support multiple uv sets base_color_texture.tex_coord()
            match texture.texture().source().source() {
                gltf::image::Source::View { view, mime_type: _ } => {
                    info!("Image source (view): {:?}", view);
                }
                gltf::image::Source::Uri { uri, mime_type: _ } => {
                    // TODO: Convert source path to a parameter that's passed in for loading mesh.
                    let path = "assets/glTF/CesiumMilkTruck/glTF/".to_owned() + uri;
                    info!("Image source (uri): {:?}", path.clone());
                    base_color_texture = ImageReader::open(path).unwrap().decode().unwrap();
                    // let base_color = base_color_texture.clone().into_rgb8().get_pixel(1, 1);
                    // let image_buffer = base_color_texture.to_rgba8();
                    // let pixels = image_buffer.as_flat_samples();
                }
            }
        }

        Self {
            indices,
            positions,
            uv,
            base_color_texture,
        }
    }

    pub fn intersect(&self, ray: &Ray, isect: &mut SurfaceInteraction) -> bool {
        let mut hit = false;
        let mut nearest_t = 9999.9;
        let mut nearest_isect = SurfaceInteraction::new();
        let max_index = self.indices.len() - 1;

        for i in (0..max_index).step_by(3) {
            let triangle = Triangle::new(
                self.positions[self.indices[i] as usize],
                self.positions[self.indices[i + 1] as usize],
                self.positions[self.indices[i + 2] as usize],
            );

            let tri_hit = triangle.intersect(ray, isect);
            if tri_hit && isect.t < nearest_t {
                hit = true;
                nearest_t = isect.t;

                nearest_isect.t = isect.t;
                nearest_isect.hit_point = isect.hit_point;
                nearest_isect.hit_normal = isect.hit_normal;

                // texture coordinates
                let st0 = self.uv[self.indices[i] as usize];
                let st1 = self.uv[self.indices[i + 1] as usize];
                let st2 = self.uv[self.indices[i + 2] as usize];
                let hit_uv = (1.0 - isect.hit_uv.x() - isect.hit_uv.y()) * st0
                    + isect.hit_uv.x() * st1
                    + isect.hit_uv.y() * st2;
                nearest_isect.hit_uv = hit_uv;
            }
        }

        isect.t = nearest_isect.t;
        isect.hit_point = nearest_isect.hit_point;
        isect.hit_normal = nearest_isect.hit_normal;
        isect.hit_uv = nearest_isect.hit_uv;
        return hit;
    }
}
