use crate::materials::matte::MatteMaterial;
use crate::materials::constant::ConstantMaterial;

#[derive(Clone)]
pub enum Material {
    Constant(ConstantMaterial),
    Matte(MatteMaterial)
}