use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;

use crate::math::Point;
use crate::math::Tuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix4x4 {
    data: [[f64; 4]; 4],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix3x3 {
    data: [[f64; 3]; 3],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix2x2 {
    data: [[f64; 2]; 2],
}

impl Matrix4x4 {
    pub fn identity() -> Matrix4x4 {
        Matrix4x4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn transpose(&self) -> Matrix4x4 {
        let mut result = Matrix4x4::identity();

        for row in 0..=3 {
            result[0][row] = self[row][0];
            result[1][row] = self[row][1];
            result[2][row] = self[row][2];
            result[3][row] = self[row][3];
        }

        result
    }
}

impl Index<usize> for Matrix4x4 {
    type Output = [f64; 4];

    fn index(&self, other: usize) -> &Self::Output {
        &self.data[other]
    }
}

impl IndexMut<usize> for Matrix4x4 {
    fn index_mut(&mut self, other: usize) -> &mut Self::Output {
        &mut self.data[other]
    }
}

impl Index<usize> for Matrix3x3 {
    type Output = [f64; 3];

    fn index(&self, other: usize) -> &Self::Output {
        &self.data[other]
    }
}

impl Index<usize> for Matrix2x2 {
    type Output = [f64; 2];

    fn index(&self, other: usize) -> &Self::Output {
        &self.data[other]
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut result = Matrix4x4::identity();

        for row in 0..=3 {
            for col in 0..=3 {
                result[row][col] = self[row][0] * other[0][col]
                    + self[row][1] * other[1][col]
                    + self[row][2] * other[2][col]
                    + self[row][3] * other[3][col]
            }
        }

        result
    }
}

impl Mul<Tuple> for Matrix4x4 {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
        Tuple::new(
            self[0][0] * other.x
                + self[0][1] * other.y
                + self[0][2] * other.z
                + self[0][3] * other.w,
            self[1][0] * other.x
                + self[1][1] * other.y
                + self[1][2] * other.z
                + self[1][3] * other.w,
            self[2][0] * other.x
                + self[2][1] * other.y
                + self[2][2] * other.z
                + self[2][3] * other.w,
            self[3][0] * other.x
                + self[3][1] * other.y
                + self[3][2] * other.z
                + self[3][3] * other.w,
        )
    }
}

impl Mul<Point> for Matrix4x4 {
    type Output = Point;

    fn mul(self, other: Self::Output) -> Self::Output {
        Point::new(
            self[0][0] * other.x + self[0][1] * other.y + self[0][2] * other.z + self[0][3],
            self[1][0] * other.x + self[1][1] * other.y + self[1][2] * other.z + self[1][3],
            self[2][0] * other.x + self[2][1] * other.y + self[2][2] * other.z + self[2][3],
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn index_accessfor_4_by_4() {
        let mat = Matrix4x4 {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [5.5, 6.7, 7.5, 8.5],
                [9.0, 10.0, 11.0, 12.0],
                [13.5, 14.5, 15.5, 16.5],
            ],
        };

        assert_approx_eq!(mat[0][0], 1.0);
        assert_approx_eq!(mat[0][3], 4.0);
        assert_approx_eq!(mat[1][0], 5.5);
        assert_approx_eq!(mat[2][2], 11.0);
        assert_approx_eq!(mat[3][0], 13.5);
        assert_approx_eq!(mat[3][2], 15.5);
    }

    #[test]
    fn index_accessfor_3_by_3() {
        let mat = Matrix3x3 {
            data: [[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]],
        };

        assert_approx_eq!(mat[0][0], -3.0);
        assert_approx_eq!(mat[1][1], -2.0);
        assert_approx_eq!(mat[2][2], 1.0);
    }

    #[test]
    fn index_accessfor_2_by_2() {
        let mat = Matrix2x2 {
            data: [[-3.0, 5.0], [1.0, -2.0]],
        };

        assert_approx_eq!(mat[0][0], -3.0);
        assert_approx_eq!(mat[0][1], 5.0);
        assert_approx_eq!(mat[1][0], 1.0);
        assert_approx_eq!(mat[1][1], -2.0);
    }

    #[test]
    fn multiply_two_matrices() {
        let mat_a = Matrix4x4 {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };
        let mat_b = Matrix4x4 {
            data: [
                [-2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, -1.0],
                [4.0, 3.0, 6.0, 5.0],
                [1.0, 2.0, 7.0, 8.0],
            ],
        };

        let result = Matrix4x4 {
            data: [
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ],
        };

        assert_eq!(result, mat_a * mat_b)
    }

    #[test]
    fn multiply_with_tuple() {
        let mat = Matrix4x4 {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(mat * tuple, Tuple::new(18.0, 24.0, 33.0, 1.0))
    }

    #[test]
    fn multiply_with_identity_matrix() {
        let mat = Matrix4x4 {
            data: [
                [0.0, 1.0, 2.0, 4.0],
                [1.0, 2.0, 4.0, 8.0],
                [2.0, 4.0, 8.0, 16.0],
                [4.0, 8.0, 16.0, 32.0],
            ],
        };

        assert_eq!(mat * Matrix4x4::identity(), mat)
    }

    #[test]
    fn multiply_identity_matrix_with_tuple() {
        let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(Matrix4x4::identity() * tuple, tuple)
    }

    #[test]
    fn transpose_matrix() {
        let mat = Matrix4x4 {
            data: [
                [0.0, 9.0, 3.0, 0.0],
                [9.0, 8.0, 0.0, 8.0],
                [1.0, 8.0, 5.0, 3.0],
                [0.0, 0.0, 5.0, 8.0],
            ],
        };

        let trans_mat = Matrix4x4 {
            data: [
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0],
            ]
        };

        assert_eq!(mat.transpose(), trans_mat);
        assert_eq!(Matrix4x4::identity().transpose(), Matrix4x4::identity());
    }
}
