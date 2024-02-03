use funty::Numeric;

pub struct Quaternion<T>
    where T: Numeric {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl<T> From<&[T; 4]> for Quaternion<T>
    where T: Numeric {
    fn from(v: &[T; 4]) -> Self {
        Self { x: v[0], y: v[1], z: v[2], w: v[3] }
    }
}