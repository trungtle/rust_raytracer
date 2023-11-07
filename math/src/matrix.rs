use std::ops;
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct SquareMatrix<T, const N: usize> {
    pub m: [[T; N]; N]
}

pub type SquareMatrix4f = SquareMatrix<f64, 4>;

impl SquareMatrix4f {
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

    pub fn from(value: f64) -> Self {
        let mut matrix = Self {
            m: [[0.0; 4]; 4]
        };
        for i in 0..4 {
            matrix.m[i][i] = value;
        }
        return matrix;
    }
}


impl ops::Mul<f64> for SquareMatrix4f {
    type Output = SquareMatrix4f;

    fn mul(self, _rhs: f64) -> Self::Output {
        let mut out_matrix = self.clone();
        for i in 0..4 {
            for j in 0..4 {
                out_matrix.m[i][j] *= _rhs;
            }
        }
        return out_matrix;
    }
}


impl ops::Mul<SquareMatrix4f> for SquareMatrix4f {
    type Output = SquareMatrix4f;

    fn mul(self, _rhs: SquareMatrix4f) -> Self::Output {
        let mut out_matrix = SquareMatrix4f::zero();
        for i in 0..4 {
            for j in 0..4 {
                out_matrix.m[i][j] += self.m[i][j] * _rhs.m[j][i];
            }
        }
        return out_matrix;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let m1 = SquareMatrix4f::identity();
        let m2 = SquareMatrix4f::from(2.0);
        let m3 = SquareMatrix4f::from(10.0);
        let m4 = m1 * m3;
        let m5 = m3 * 2.0;
        let m6 = m2 * m3;
        assert_eq!(m3, m4);
        assert_eq!(m5, m6);
    }
}


