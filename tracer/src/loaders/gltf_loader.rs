use gltf;
use log::{info, debug};

pub struct GData {
    pub doc: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}

pub fn load_gltf(path: &str) -> GData
{
    let (doc, buffers, images) = gltf::import(path).unwrap();
    info!("Node");
    for node in doc.nodes() {
        info!("{:?}", node.name());
        info!("{:?}", node.transform());
        if let Some(mesh) = node.mesh() {
            info!("Mesh #{} {:?}", mesh.index(), mesh.name());
            for primitive in mesh.primitives() {
                info!("- Primitive #{}", primitive.index());
        
                    // Attributes
                    for (semantic, _) in primitive.attributes() {
                        info!("-- {:?}", semantic);
                    }
        
                    // Positions
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(iter) = reader.read_positions() {
                    for vertex_position in iter {
                        debug!("{:?}", vertex_position);
                    }
                }
        
                // Indices
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(iter) = reader.read_indices() {
                    for index in iter.into_u32() {
                        debug!("{:?}", index);
                    }
                }
            }        
        }
    }
    
    info!("Materials:");
    for material in doc.materials() {
        info!("-- {:?}", material.name());
    }

    info!("Textures:");
    for texture in doc.textures() {
        info!("-- {:?}", texture.name());
    }

    info!("Images:"); 
    for image in doc.images() {
        info!("-- {:?}", image.name());
    }

    info!("Buffers:"); 
    for buffer in doc.buffers() {
        info!("-- {:?}", buffer.name());
    }

    return GData { doc, buffers, images };
}