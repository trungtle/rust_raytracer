use std::sync::Arc;
use std::ops;

use crate::math::{Float, Vec3, Mat4};

#[derive(Clone)]
pub struct Transform {
    pub matrix: Mat4,
    pub matrix_inv: Mat4,
    pub parent: Option<Arc<Transform>>
}

impl std::default::Default for Transform {
    fn default() -> Self {
        Self {
            matrix: Mat4::identity(),
            matrix_inv: Mat4::identity(),
            parent: None
        }
    }
}

impl Transform {
    pub fn new(position: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        let mut out_transform = Transform::default();
        out_transform =
            Transform::translate(position) *
            Transform::rotate_x(rotation.x) *
            Transform::rotate_x(rotation.y) *
            Transform::rotate_x(rotation.z) *
            Transform::scale(scale);
        return out_transform;
    }

    pub fn from_matrix(mat: Mat4) -> Self {
        Self {
            matrix: mat,
            matrix_inv: mat,
            parent: None
        }
    }

    pub fn from_array(array: [[Float; 4]; 4]) -> Self {
        Self {
            matrix: Mat4::from_array(array),
            matrix_inv: Mat4::from_array(array),
            parent: None
        }
    }

    pub fn get_position(&self) -> Vec3 {
        Vec3::new(self.matrix[[3,0]], self.matrix[[3,1]], self.matrix[[3,2]])
    }

    pub fn get_scale(&self) -> Vec3 {
        Vec3::new(self.matrix[[0,0]], self.matrix[[1,1]], self.matrix[[2,2]])
    }

    pub fn translate(position: Vec3) -> Self {
        let mut out_transform = Transform::default();
        out_transform.matrix[[3, 0]] = position[0];
        out_transform.matrix[[3, 1]] = position[1];
        out_transform.matrix[[3, 2]] = position[2];

        out_transform.matrix_inv[[3, 0]] = -position[0];
        out_transform.matrix_inv[[3, 1]] = -position[1];
        out_transform.matrix_inv[[3, 2]] = -position[2];
        return out_transform;
    }

    pub fn rotate_x(theta_radian: Float) -> Self {
        let sintheta = theta_radian.sin();
        let costheta = theta_radian.sin();
        let out_transform = Transform::from_array(
            [[1., 0., 0., 0.],
            [0., costheta, -sintheta, 0.],
            [0., sintheta, costheta, 0.],
            [0., 0., 0., 1.]]);
        return out_transform;
    }

    pub fn rotate_y(theta_radian: Float) -> Self {
        let sintheta = theta_radian.sin();
        let costheta = theta_radian.sin();
        let out_transform = Transform::from_array(
            [[costheta, 0., sintheta, 0.],
            [0., 1., 0., 0.],
            [-sintheta, 0., costheta, 0.],
            [0., 0., 0., 1.]]);
        return out_transform;
    }

    pub fn rotate_z(theta_radian: Float) -> Self {
        let sintheta = theta_radian.sin();
        let costheta = theta_radian.sin();
        let out_transform = Transform::from_array(
            [[costheta, -sintheta, 0., 0.],
            [sintheta, costheta, 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.]]);
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