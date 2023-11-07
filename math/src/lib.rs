pub mod matrix;
pub mod point;
pub mod tuple;
pub mod vector;

pub use vector::{Vec2, Vec3};
pub use tuple::{Tuple2d, Tuple3d, Tuple4d};
pub use matrix::SquareMatrix4f;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
