pub mod quaternion;
pub mod matrix;
pub mod point;
pub mod tuple;
pub mod vector;

pub use vector::{Vector2, Vector3};
pub use tuple::{Tuple2d, Tuple3d, Tuple4d};
pub use matrix::Matrix4;
pub use funty::Numeric as Numeric;
pub use funty::Floating as Floating;
pub use quaternion::Quaternion;

pub type Mat4 = Matrix4<Float>;
pub type Vec3 = Vector3<Float>;
pub type Vec2 = Vector2<Float>;
pub type Float = f32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
