pub mod constant;

use crate::core::interaction::SurfaceInteraction;

pub trait Texture<T> {
    fn evaluate(&self, si: &SurfaceInteraction) -> T;
}