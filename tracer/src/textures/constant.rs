use crate::core::interaction::SurfaceInteraction;
use crate::core::texture::Texture;

#[derive(Clone)]
pub struct ConstantTexture<T> {
    pub value: T,
}

impl<T: Copy> ConstantTexture<T> {
    pub fn new(value: T) -> Self {
        ConstantTexture { value }
    }
}

impl<T: Copy> Texture<T> for ConstantTexture<T> {
    fn evaluate(&self, _si: &SurfaceInteraction) -> T {
        self.value
    }
}