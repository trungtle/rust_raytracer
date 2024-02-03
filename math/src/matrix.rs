use std::ops;
use std::ops::Mul;

use crate::{Numeric, Float};
use crate::Vector3;
use crate::Quaternion;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct SquareMatrix<T, const N: usize>
    where T: Numeric {
    pub m: [[T; N]; N]
}

pub type Matrix4<T: Numeric> = SquareMatrix<T, 4>;

impl Matrix4<Float> {
    pub fn identity() -> Self {
        let mut matrix = Self {
            m: [[0.0; 4]; 4]
        };
        for i in 0..4 {
            matrix.m[i][i] = 1.0;
        }
        return matrix;
    }

    pub fn from_quat(quat: Quaternion<Float>) -> Self {
        let mut matrix = SquareMatrix::identity();

        let q0 = quat.x;
        let q1 = quat.y;
        let q2 = quat.z;
        let q3 = quat.w;
        let q0_sq = quat.x * quat.x;
        let q1_sq = quat.y * quat.y;
        let q2_sq = quat.z * quat.z;
        let q3_sq = quat.w * quat.w;
        let q0q1_2 = 2.*q0*q1;
        let q0q2_2 = 2.*q0*q2;
        let q0q3_2 = 2.*q0*q3;
        let q1q2_2 = 2.*q1*q2;
        let q1q3_2 = 2.*q1*q3;
        let q2q3_2 = 2.*q2*q3;
        matrix.m[0][0] = q0_sq + q1_sq - q2_sq - q3_sq;
        matrix.m[0][1] = q1q2_2 - q0q3_2;
        matrix.m[0][2] = q1q3_2 + q0q2_2;
        matrix.m[1][0] = q1q2_2 + q0q3_2;
        matrix.m[1][1] = q0_sq - q1_sq + q2_sq - q3_sq;
        matrix.m[1][2] = q2q3_2 - q0q1_2;
        matrix.m[2][0] = q1q3_2 - q0q2_2;
        matrix.m[2][1] = q2q3_2 + q0q1_2;
        matrix.m[2][2] = q0_sq - q1_sq - q2_sq + q3_sq;

        return matrix;
    }

}

impl<T> Matrix4<T>
    where T: Numeric + Mul<T> {
    pub fn zero() -> Self {
        Self {
            m: [[T::default(); 4]; 4]
        }
    }

    pub fn from(value: T) -> Self {
        let mut matrix = Self {
            m: [[T::default(); 4]; 4]
        };
        for i in 0..4 {
            matrix.m[i][i] = value;
        }
        return matrix;
    }

    pub fn from_array(value: [[T; 4]; 4]) -> Self {
        Self {
            m: value
        }
    }
}

// 1st index is row, 2nd index is column
impl<T> std::ops::Index<[usize; 2]> for Matrix4<T>
    where T: Numeric {
    type Output = T;

    fn index(&self, idx: [usize; 2]) -> &T {
        &self.m[idx[0]][idx[1]]
    }
}

// 1st index is row, 2nd index is column
impl<T> std::ops::IndexMut<[usize; 2]> for Matrix4<T>
    where T: Numeric {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut T {
        &mut self.m[idx[0]][idx[1]]
    }
}

impl<T> ops::Mul<T> for Matrix4<T>
    where T: Numeric {
    type Output = Matrix4<T>;

    fn mul(self, _rhs: T) -> Self::Output {
        let mut out_matrix = self.clone();
        for i in 0..4 {
            for j in 0..4 {
                out_matrix.m[i][j] *= _rhs;
            }
        }
        return out_matrix;
    }
}


impl<T> ops::Mul<Matrix4<T>> for Matrix4<T>
    where T: Numeric {
    type Output = Matrix4<T>;

    fn mul(self, _rhs: Matrix4<T>) -> Self::Output {
        let mut out_matrix = Matrix4::zero();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    out_matrix[[i, j]] += self[[i, k]] * _rhs[[k, j]]
                }
            }
        }
        return out_matrix;
    }
}

impl<T> ops::Mul<Vector3<T>> for Matrix4<T>
    where T: Numeric {
    type Output = Vector3<T>;

    fn mul(self, _rhs: Vector3<T>) -> Self::Output {
        let mut out_vector = Vector3::default();
        for i in 0..3 {
            for j in 0..4 {
                if j >= 3 {
                    out_vector[i] += self[[i,j]];
                } else {
                    out_vector[i] += self[[i,j]] * _rhs[j];
                }
            }
        }
        return out_vector;
    }
}

impl<T> ops::Mul<&mut Vector3<T>> for Matrix4<T>
    where T: Numeric {
    type Output = Vector3<T>;

    fn mul(self, _rhs: &mut Vector3<T>) -> Self::Output {
        let mut out_vector = Vector3::default();
        for i in 0..3 {
            for j in 0..4 {
                if j >= 3 {
                    out_vector[i] += self[[i,j]];
                } else {
                    out_vector[i] += self[[i,j]] * _rhs[j];
                }
            }
        }
        return out_vector;
    }
}

#[cfg(test)]
mod tests {
    use crate::Float;

    use super::*;

    fn temp_mat1() -> Matrix4<Float> {
        Matrix4::<Float>::from_array([
            [-4.,	2.,	1.,	-1.],
            [4.,	5.,	3.,	-3.],
            [6.,	7.,	0.,	8.],
            [9.,	10.,	11., 12.]])
    }

    fn temp_mat2() -> Matrix4<Float> {
        Matrix4::<Float>::from_array([
            [-2.,	2.,	-2.,	2.],
            [4.,	5.,	4.,	5.],
            [6.,	7.,	6.,	7.],
            [9.,	10.,	9., 10.]])
    }


    #[test]
    fn test_matrix_index() {
        let m1 = temp_mat1();
        // 1st index is row, 2nd index is column
        assert_eq!(7.0, m1[[2, 1]]);
        assert_eq!(3.0, m1[[1, 2]]);
    }

    #[test]
    fn test_matrix_mul_matrix() {
        let m_identity = Matrix4::identity();
        let m1 = temp_mat1();
        let m2 = temp_mat2();

        let m1xm2 = Matrix4::<Float>::from_array([
            [13.,	-1.,	13.,	-1.],
            [3.,	24.,	3.,	    24.],
            [88.,	127.,	88.,	127.],
            [196.,	265.,	196.,   265.]]);


        assert_eq!(m1, m1 * m_identity);
        assert_eq!(m2, m2 * m_identity);
        assert_eq!(m1xm2, m1 * m2);

    }

    #[test]
    fn test_matrix_mul_vector() {
        let m1 = temp_mat1();

        let v1 = Vector3 {x: 1.0, y: 2.0, z: 3.0 };
        let m1xv1 = Vector3 {x: 2., y: 20., z: 28. };
        assert_eq!(m1 * v1, m1xv1);
    }
}

