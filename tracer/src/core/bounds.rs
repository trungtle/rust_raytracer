use math::Vec3;

#[derive(Clone, PartialEq, Debug)]
pub struct Bounds3f {
    pub p_min: Vec3,
    pub p_max: Vec3, 
}