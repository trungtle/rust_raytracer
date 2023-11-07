#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Tuple<T, const N: usize> {
    v: [T; N]
}

pub type Tuple2d = Tuple<f64, 2>;
pub type Tuple3d = Tuple<f64, 3>;
pub type Tuple4d = Tuple<f64, 4>;

impl<const N: usize> Tuple<f64, N> {
    pub fn zero() -> Self {
        Self {
            v: [0.0; N]
        }
    }

    pub fn from(val: f64) -> Self {
        Self {
            v: [val; N]
        }
    }
}

impl Tuple2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            v: [x, y]
        }
    }
}

impl Tuple3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            v: [x, y, z]
        }
    }
}

impl Tuple4d {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            v: [x, y, z, w]
        }
    }
}

