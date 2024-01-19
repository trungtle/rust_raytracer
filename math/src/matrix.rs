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

// 1st index is row, 2nd index is column
impl std::ops::Index<[usize; 2]> for Mat4 {
    type Output = Float;

    fn index(&self, idx: [usize; 2]) -> &Float {
        &self.m[idx[0]][idx[1]]
    }
}

// 1st index is row, 2nd index is column
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
                for k in 0..4 {
                    out_matrix[[i, j]] += self[[i, k]] * _rhs[[k, j]]
                }
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

impl ops::Mul<&mut Vec3> for Mat4   {
    type Output = Vec3;

    fn mul(self, _rhs: &mut Vec3) -> Self::Output {
        let mut out_vector = Vec3::from(0.);
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
    use super::*;

    fn temp_mat1() -> Mat4 {
        Mat4::from_array([
            [-4.,	2.,	1.,	-1.],
            [4.,	5.,	3.,	-3.],
            [6.,	7.,	0.,	8.],
            [9.,	10.,	11., 12.]])
    }

    fn temp_mat2() -> Mat4 {
        Mat4::from_array([
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
        let m_identity = Mat4::identity();
        let m1 = temp_mat1();
        let m2 = temp_mat2();

        let m1xm2 = Mat4::from_array([
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

        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let m1xv1 = Vec3::new(2., 20., 28.);
        assert_eq!(m1 * v1, m1xv1);
    }
}

