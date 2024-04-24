use math::Vec3;

pub struct Bounds3 {
    pub min: Vec3,
    pub max: Vec3,
}

impl Bounds3 {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
}

// Orthonormal basics
pub struct ONB {
    axes: [Vec3; 3],
}

impl ONB {
    fn u(&self) -> Vec3 {
        self.axes[0]
    }
    fn v(&self) -> Vec3 {
        self.axes[1]
    }
    fn w(&self) -> Vec3 {
        self.axes[2]
    }
    // Build from a w vector

    pub fn from(w: &Vec3) -> Self {
        let mut axes = [Vec3::from(0.); 3];
        axes[2] = w.normalize();

        // pick a vector not parallel to n
        let a = if axes[2].x.abs() > 0.9 {
            Vec3 {
                x: 0.,
                y: 1.,
                z: 0.,
            }
        } else {
            Vec3 {
                x: 1.,
                y: 0.,
                z: 0.,
            }
        };
        axes[1] = Vec3::cross(axes[2], a);
        axes[0] = Vec3::cross(axes[2], axes[1]);

        Self { axes }
    }

    pub fn from_local(&self, n: &Vec3) -> Vec3 {
        n.x * self.u() + n.y * self.v() + n.z * self.w()
    }
}
