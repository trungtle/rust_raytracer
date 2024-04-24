use std::ops;

use funty::Numeric;
use math::{Float, Mat4, Matrix4, Quaternion, Vec3};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Transform {
    pub matrix: Mat4,
    pub matrix_inv: Mat4,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            matrix: Mat4::identity(),
            matrix_inv: Mat4::identity(),
        }
    }
}

impl From<&Quaternion<Float>> for Transform {
    fn from(quat: &Quaternion<Float>) -> Self {
        Self {
            matrix: Mat4::from(quat),
            // TODO: Apply inverse
            matrix_inv: Mat4::from(quat),
        }
    }
}

impl From<&Mat4> for Transform {
    fn from(mat: &Mat4) -> Self {
        Self {
            matrix: mat.clone(),
            // TODO: Apply inverse
            matrix_inv: mat.clone(),
        }
    }
}

impl From<&gltf::scene::Transform> for Transform {
    fn from(gltf_xform: &gltf::scene::Transform) -> Self {
        let mut matrix = Transform::default();
        match gltf_xform {
            gltf::scene::Transform::Matrix { matrix } => {}
            gltf::scene::Transform::Decomposed {
                translation,
                rotation,
                scale,
            } => {
                matrix = Transform::translate(Vec3::from(translation))
                    * Transform::from(&Quaternion::from(rotation))
                    * Transform::scale(Vec3::from(scale));
            }
        }

        // TODO: Apply inverse
        return matrix;
    }
}

impl Transform {
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        let mut out_transform = Transform::default();
        out_transform = Transform::translate(position)
            * Transform::rotate_x(rotation.x)
            * Transform::rotate_x(rotation.y)
            * Transform::rotate_x(rotation.z)
            * Transform::scale(scale);

        // TODO: Apply inverse
        return out_transform;
    }

    pub fn from_array(array: [[Float; 4]; 4]) -> Self {
        Self {
            matrix: Mat4::from_array(array),
            // TODO: Apply inverse
            matrix_inv: Mat4::from_array(array),
        }
    }

    pub fn get_position(&self) -> Vec3 {
        Vec3 {
            x: self.matrix[[0, 3]],
            y: self.matrix[[1, 3]],
            z: self.matrix[[2, 3]],
        }
    }

    pub fn get_scale(&self) -> Vec3 {
        Vec3 {
            x: self.matrix[[0, 0]],
            y: self.matrix[[1, 1]],
            z: self.matrix[[2, 2]],
        }
    }

    pub fn translate(position: Vec3) -> Self {
        let mut out_transform = Transform::default();
        out_transform.matrix[[0, 3]] = position[0];
        out_transform.matrix[[1, 3]] = position[1];
        out_transform.matrix[[2, 3]] = position[2];

        out_transform.matrix_inv[[0, 3]] = -position[0];
        out_transform.matrix_inv[[1, 3]] = -position[1];
        out_transform.matrix_inv[[2, 3]] = -position[2];

        // TODO: Apply inverse
        return out_transform;
    }

    pub fn rotate_x(theta_radian: Float) -> Self {
        let sintheta = theta_radian.sin();
        let costheta = theta_radian.cos();
        let out_transform = Transform::from_array([
            [1., 0., 0., 0.],
            [0., costheta, -sintheta, 0.],
            [0., sintheta, costheta, 0.],
            [0., 0., 0., 1.],
        ]);

        // TODO: Apply inverse
        return out_transform;
    }

    pub fn rotate_y(theta_radian: Float) -> Self {
        let sintheta = theta_radian.sin();
        let costheta = theta_radian.cos();
        let out_transform = Transform::from_array([
            [costheta, 0., sintheta, 0.],
            [0., 1., 0., 0.],
            [-sintheta, 0., costheta, 0.],
            [0., 0., 0., 1.],
        ]);

        // TODO: Apply inverse
        return out_transform;
    }

    pub fn rotate_z(theta_radian: Float) -> Self {
        let sintheta = theta_radian.sin();
        let costheta = theta_radian.cos();
        let out_transform = Transform::from_array([
            [costheta, -sintheta, 0., 0.],
            [sintheta, costheta, 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);

        // TODO: Apply inverse
        return out_transform;
    }

    pub fn scale(scale: Vec3) -> Self {
        let mut out_transform = Transform::default();
        out_transform.matrix[[0, 0]] = scale[0];
        out_transform.matrix[[1, 1]] = scale[1];
        out_transform.matrix[[2, 2]] = scale[2];

        out_transform.matrix_inv[[0, 0]] = 1.0 / scale[0];
        out_transform.matrix_inv[[1, 1]] = 1.0 / scale[1];
        out_transform.matrix_inv[[2, 2]] = 1.0 / scale[2];

        // TODO: Apply inverse
        return out_transform;
    }
}

impl ops::Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, _rhs: Transform) -> Self::Output {
        let mut out_transform = self.clone();
        out_transform.matrix = out_transform.matrix * _rhs.matrix;
        return out_transform;
    }
}

// -----------------------------------------------------------------------------
// Format
// -----------------------------------------------------------------------------
impl std::fmt::Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position: {}, Scale: {}",
            self.get_position(),
            self.get_scale()
        )
    }
}
