use crate::materials::matte::MatteMaterial;
use crate::materials::constant::ConstantMaterial;
use crate::materials::MetalMaterial;

#[derive(Clone)]
pub enum Material {
    Constant(ConstantMaterial),
    Metal(MetalMaterial),
    Matte(MatteMaterial)
}