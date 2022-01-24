use gltf;

pub struct GData {
    pub doc: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}

pub fn load_gltf(path: &str) -> GData
{
    let (doc, buffers, images) = gltf::import(path).unwrap();
    let g_data = GData { doc, buffers, images };
    for mesh in g_data.doc.meshes() {
       println!("Mesh #{}", mesh.index());
       for primitive in mesh.primitives() {
           println!("- Primitive #{}", primitive.index());

            // Attributes
            for (semantic, _) in primitive.attributes() {
                println!("-- {:?}", semantic);
            }

            // Positions
           let reader = primitive.reader(|buffer| Some(&g_data.buffers[buffer.index()]));
           if let Some(iter) = reader.read_positions() {
               for vertex_position in iter {
                   println!("{:?}", vertex_position);
               }
           }
           
           // Indices
           let reader = primitive.reader(|buffer| Some(&g_data.buffers[buffer.index()]));
           if let Some(iter) = reader.read_indices() {
               for index in iter.into_u32() {
                   println!("{:?}", index);
               }
           }
       }
    }

    return g_data;
}