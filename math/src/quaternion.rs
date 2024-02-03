pub struct Quaternion<T>
    where T: Copy {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl<T> Quaternion<T>
    where T: Copy {
    pub fn new(v: &[T; 4]) -> Self {
        Self { x: v[0], y: v[1], z: v[2], w: v[3] }
    }
}