use std::ops;
use std::ops::{Index, IndexMut};

use crate::types::Float;
use crate::Vec3;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct SquareMatrix<T, const N: usize> {
    pub m: [[T; N]; N]
}

pub type Mat4 = SquareMatrix<Float, 4>;

impl Mat4 {
    pub fn zero() -> Self {
        Self {
            m: [[0.0; 4]; 4]
        }
    }

    pub fn identity() -> Self {
        let mut matrix = Self {
            m: [[0.0; 4]; 4]
        };
        for i in 0..4 {
            matrix.m[i][i] = 1.0;
        }
        return matrix;
    }

    pub fn from(value: Float) -> Self {
        let mut matrix = Self {
            m: [[0.0; 4]; 4]
        };
        for i in 0..4 {
            matrix.m[i][i] = value;
        }
        return matrix;
    }

    pub fn from_array(value: [[Float; 4]; 4]) -> Self {
        Self {
            m: value
        }
    }
}

impl std::ops::Index<[usize; 2]> for Mat4 {
    type Output = Float;

    fn index(&self, idx: [usize; 2]) -> &Float {
        &self.m[idx[0]][idx[1]]
    }
}

impl std::ops::IndexMut<[usize; 2]> for Mat4 {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Float {
        &mut self.m[idx[0]][idx[1]]
    }
}

impl ops::Mul<Float> for Mat4 {
    type Output = Mat4;

    fn mul(self, _rhs: Float) -> Self::Output {
        let mut out_matrix = self.clone();
        for i in 0..4 {
            for j in 0..4 {
                out_matrix.m[i][j] *= _rhs;
            }
        }
        return out_matrix;
    }
}


impl ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, _rhs: Mat4) -> Self::Output {
        let mut out_matrix = Mat4::zero();
        for i in 0..4 {
            for j in 0..4 {
                out_matrix.m[i][j] += self.m[i][j] * _rhs.m[j][i];
            }
        }
        return out_matrix;
    }
}

impl ops::Mul<Vec3> for Mat4   {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Self::Output {
        let mut out_vector = Vec3::from(0.);
        for i in 0..3 {
            for j in 0..3 {
                out_vector[i] += self.m[i][j] * _rhs[i];
            }
        }
        return out_vector;
    }
}

impl ops::Mul<&mut Vec3> for Mat4   {
    type Output = Vec3;

    fn mul(self, _rhs: &mut Vec3) -> Self::Output {
        let mut out_vector = Vec3::from(0.);
        for i in 0..3 {
            for j in 0..3 {
                out_vector[i] += self.m[j][i] * _rhs[i];
            }
        }
        return out_vector;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        let m1 = Mat4::identity();
        let m2 = Mat4::from(2.0);
        let m3 = Mat4::from(10.0);
        let m4 = m1 * m3;
        let m5 = m3 * 2.0;
        let m6 = m2 * m3;

        assert_eq!(m3, m4);
        assert_eq!(m5, m6);

    }

    #[test]
    fn test_matrix_mul_vector() {
        let m1 = Mat4::identity();

        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let m1_x_v1 = m1 * v1;
        assert_eq!(v1, m1_x_v1);
    }
}


