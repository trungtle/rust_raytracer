pub mod matte;
pub mod pdf;

pub use matte::MatteMaterial as MatteMaterial;
pub use pdf::Pdf as Pdf;
pub use pdf::UniformPdf as UniformPdf;

use crate::math::Vec3;

// Orthonormal basics
pub struct ONB {
    axes: [Vec3; 3]
}

impl ONB {
    fn u(&self) -> Vec3 { self.axes[0] }
    fn v(&self) -> Vec3 { self.axes[1] }
    fn w(&self) -> Vec3 { self.axes[2] }
    // Build from a w vector
    
    fn from(w: &Vec3) -> Self {
        let mut axes = [Vec3::from(0.); 3];
        axes[2] = w.normalize();

        // pick a vector not parallel to n
        let a = if axes[2].x.abs() > 0.9 { Vec3::new(0., 1., 0.) } else { Vec3::new(1., 0., 0.) };
        axes[1] = Vec3::cross(axes[2], a);
        axes[0] = Vec3::cross(axes[2], axes[1]);

        Self {
            axes
        }
    }
    
    fn from_local(&self, n: &Vec3) -> Vec3 {
        n.x * self.u() + n.y * self.v() + n.z * self.w()
    }
}

