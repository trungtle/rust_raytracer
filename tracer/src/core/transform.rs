use crate::math::SquareMatrix4f;

#[derive(Clone)]
pub struct Transform {
    pub local_to_world: SquareMatrix4f,
    pub world_to_local: SquareMatrix4f
}

impl std::default::Default for Transform {
    fn default() -> Self {
        Self {
            local_to_world: SquareMatrix4f::identity(),
            world_to_local: SquareMatrix4f::identity()
        }
    }
}