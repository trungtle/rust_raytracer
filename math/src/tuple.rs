use crate::Float;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Tuple<T, const N: usize> {
    v: [T; N],
}

pub type Tuple2d = Tuple<Float, 2>;
pub type Tuple3d = Tuple<Float, 3>;
pub type Tuple4d = Tuple<Float, 4>;

impl<const N: usize> Tuple<Float, N> {
    pub fn zero() -> Self {
        Self { v: [0.0; N] }
    }

    pub fn from(val: Float) -> Self {
        Self { v: [val; N] }
    }
}

impl Tuple2d {
    pub fn new(x: Float, y: Float) -> Self {
        Self { v: [x, y] }
    }
}

impl Tuple3d {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { v: [x, y, z] }
    }
}

impl Tuple4d {
    pub fn new(x: Float, y: Float, z: Float, w: Float) -> Self {
        Self { v: [x, y, z, w] }
    }
}
