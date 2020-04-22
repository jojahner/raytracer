use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Mul;

use crate::math::Point;
use crate::math::Tuple;
use crate::math::Vector;

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

    pub fn translation(x: f64, y: f64, z: f64) -> Matrix4x4 {
        Matrix4x4 {
            data: [
                [1.0, 0.0, 0.0,   x],
                [0.0, 1.0, 0.0,   y],
                [0.0, 0.0, 1.0,   z],
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

    pub fn submatrix(&self, row_to_remove: usize, column_to_remove: usize) -> Matrix3x3 {
        // TODO: figure out how this generic stuff works an then extract this!
        let vec: Vec<Vec<f64>> = self
            .data
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != row_to_remove)
            .map(|(_, e)| {
                e.iter()
                    .enumerate()
                    .filter(|&(i, _)| i != column_to_remove)
                    .map(|(_, e)| *e)
                    .collect()
            })
            .collect();

        Matrix3x3 {
            data: [
                [vec[0][0], vec[0][1], vec[0][2]],
                [vec[1][0], vec[1][1], vec[1][2]],
                [vec[2][0], vec[2][1], vec[2][2]],
            ],
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut det: f64 = 0.0;
        for col in 0..=3 {
            det += self[0][col] * self.cofactor(0, col)
        }
        det
    }

    pub fn is_invertiable(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Option<Matrix4x4> {
        let mut data: [[f64; 4]; 4] = [[0.0; 4]; 4];
        let det = self.determinant();

        if det == 0.0 { return None; }

        for row in 0..=3 {
            for col in 0..=3 {
                let c = self.cofactor(row, col);
                data[col][row] = c / det;
            }
        }

        Some(Matrix4x4 {
            data: [
                [data[0][0], data[0][1], data[0][2], data[0][3]],
                [data[1][0], data[1][1], data[1][2], data[1][3]],
                [data[2][0], data[2][1], data[2][2], data[2][3]],
                [data[3][0], data[3][1], data[3][2], data[3][3]],
            ],
        })
    }
}

impl Matrix3x3 {
    pub fn submatrix(&self, row_to_remove: usize, column_to_remove: usize) -> Matrix2x2 {
        // TODO: figure out how this generic stuff works an then extract this!
        let vec: Vec<Vec<f64>> = self
            .data
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != row_to_remove)
            .map(|(_, e)| {
                e.iter()
                    .enumerate()
                    .filter(|&(i, _)| i != column_to_remove)
                    .map(|(_, e)| *e)
                    .collect()
            })
            .collect();

        Matrix2x2 {
            data: [[vec[0][0], vec[0][1]], [vec[1][0], vec[1][1]]],
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut det: f64 = 0.0;
        for col in 0..=2 {
            det += self[0][col] * self.cofactor(0, col)
        }
        det
    }
}

impl Matrix2x2 {
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
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

impl Mul<Vector> for Matrix4x4 {
    type Output = Vector;

    fn mul(self, other: Self::Output) -> Self::Output {
        Vector::new(
            self[0][0] * other.x + self[0][1] * other.y + self[0][2] * other.z,
            self[1][0] * other.x + self[1][1] * other.y + self[1][2] * other.z,
            self[2][0] * other.x + self[2][1] * other.y + self[2][2] * other.z,
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
            ],
        };

        assert_eq!(mat.transpose(), trans_mat);
        assert_eq!(Matrix4x4::identity().transpose(), Matrix4x4::identity());
    }

    #[test]
    fn determinant_of_a_2_by_2_matrix() {
        let mat = Matrix2x2 {
            data: [[1.0, 5.0], [-3.0, 2.0]],
        };

        assert_approx_eq!(mat.determinant(), 17.0)
    }

    #[test]
    fn submatrix_of_a_3_by_3_matrix() {
        let mat = Matrix3x3 {
            data: [[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]],
        };

        let result = Matrix2x2 {
            data: [[-3.0, 2.0], [0.0, 6.0]],
        };

        assert_eq!(mat.submatrix(0, 2), result)
    }

    #[test]
    fn submatrix_of_a_4_by_4_matrix() {
        let mat = Matrix4x4 {
            data: [
                [-6.0, 1.0, 1.0, 6.0],
                [-8.0, 5.0, 8.0, 6.0],
                [-1.0, 0.0, 8.0, 2.0],
                [-7.0, 1.0, -1.0, 1.0],
            ],
        };

        let result = Matrix3x3 {
            data: [[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]],
        };

        assert_eq!(mat.submatrix(2, 1), result)
    }

    #[test]
    fn calculate_minor_for_3_by_3_matrix() {
        let mat = Matrix3x3 {
            data: [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]],
        };

        assert_approx_eq!(mat.minor(1, 0), 25.0)
    }

    #[test]
    fn cofactor_of_a_3_by_3_matrix() {
        let mat = Matrix3x3 {
            data: [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]],
        };

        assert_approx_eq!(mat.cofactor(0, 0), -12.0);
        assert_approx_eq!(mat.minor(0, 0), -12.0);
        assert_approx_eq!(mat.cofactor(1, 0), -25.0);
        assert_approx_eq!(mat.minor(1, 0), 25.0);
    }

    #[test]
    fn determinant_of_a_3_by_3_matrix() {
        let mat = Matrix3x3 {
            data: [[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]],
        };

        assert_approx_eq!(mat.cofactor(0, 0), 56.0);
        assert_approx_eq!(mat.cofactor(0, 1), 12.0);
        assert_approx_eq!(mat.cofactor(0, 2), -46.0);
        assert_approx_eq!(mat.determinant(), -196.0);
    }

    #[test]
    fn determinant_of_a_4_by_4_matrix() {
        let mat = Matrix4x4 {
            data: [
                [-2.0, -8.0, 3.0, 5.0],
                [-3.0, 1.0, 7.0, 3.0],
                [1.0, 2.0, -9.0, 6.0],
                [-6.0, 7.0, 7.0, -9.0],
            ],
        };

        assert_approx_eq!(mat.cofactor(0, 0), 690.0);
        assert_approx_eq!(mat.cofactor(0, 1), 447.0);
        assert_approx_eq!(mat.cofactor(0, 2), 210.0);
        assert_approx_eq!(mat.cofactor(0, 3), 51.0);
        assert_approx_eq!(mat.determinant(), -4071.0);
    }

    #[test]
    fn test_4_by_4_is_invertiable() {
        let mat = Matrix4x4 {
            data: [
                [6.0, 4.0, 4.0, 4.0],
                [5.0, 5.0, 7.0, 6.0],
                [4.0, -9.0, 3.0, -7.0],
                [9.0, 1.0, 7.0, -6.0]
            ]
        };
        assert_approx_eq!(mat.determinant(), -2120.0);
        assert_eq!(mat.is_invertiable(), true)
    }

    #[test]
    fn test_4_by_4_is_not_invertiable() {
        let mat = Matrix4x4 {
            data: [
                [-4.0, 2.0, -2.0, -3.0],
                [9.0, 6.0, 2.0, 6.0],
                [0.0, -5.0, 1.0, -5.0],
                [0.0, 0.0, 0.0, 0.0]
            ]
        };
        assert_approx_eq!(mat.determinant(), 0.0);
        assert_eq!(mat.is_invertiable(), false)
    }

    #[test]
    fn inverse_of_4_by_4_matrix() {
        let mat = Matrix4x4 {
            data: [
                [-5.0, 2.0, 6.0, -8.0],
                [1.0, -5.0, 1.0, 8.0],
                [7.0, 7.0, -6.0, -7.0],
                [1.0, -3.0, 7.0, 4.0]
            ]
        };

        let inverted_mat = Matrix4x4 {
            data: [
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639]

            ],
        };
        let inverse = mat.inverse().unwrap();

        assert_approx_eq!(mat.determinant(), 532.0);
        assert_approx_eq!(mat.cofactor(2, 3), -160.0);
        assert_approx_eq!(inverse.data[3][2], -160.0/532.0);
        assert_approx_eq!(mat.cofactor(3, 2), 105.0);
        assert_approx_eq!(inverse.data[2][3], 105.0/532.0);

        assert_approx_4_by_4_eq!(inverse, inverted_mat)
    }

    #[test]
    fn another_inverse_of_4_by_4_matrix() {
        let mat_a = Matrix4x4 {
            data: [
                [ 8.0, -5.0,  9.0,  2.0],
                [ 7.0,  5.0,  6.0,  1.0],
                [-6.0,  0.0,  9.0,  6.0],
                [-3.0,  0.0, -9.0, -4.0]
            ]
        };

        let inverted_mat_a = Matrix4x4 {
            data: [
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692,  0.12308,  0.02564,  0.03077],
                [ 0.35897,  0.35897,  0.43590,  0.92308],
                [-0.69231, -0.69231, -0.76923,  -1.92308]
            ],
        };
        let inverse_a = mat_a.inverse().unwrap();
        assert_approx_4_by_4_eq!(inverse_a, inverted_mat_a);

        let mat_b = Matrix4x4 {
            data: [
                [ 9.0,  3.0,  0.0,  9.0],
                [-5.0, -2.0, -6.0, -3.0],
                [-4.0,  9.0,  6.0,  4.0],
                [-7.0,  6.0,  6.0,  2.0]
            ]
        };

        let inverted_mat_b = Matrix4x4 {
            data: [
                [-0.04074, -0.07778,  0.14444, -0.22222],
                [-0.07778,  0.03333,  0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926,  0.12963],
                [ 0.17778,  0.06667, -0.26667,  0.33333]
            ],
        };
        let inverse_b = mat_b.inverse().unwrap();
        assert_approx_4_by_4_eq!(inverse_b, inverted_mat_b)
    }

    #[test]
    fn multiply_inverse_4_by_4_matrix() {
        let mat_a = Matrix4x4 {
            data: [
                [ 3.0, -9.0,  7.0,  3.0],
                [ 3.0, -8.0,  2.0, -9.0],
                [-4.0,  4.0,  4.0,  1.0],
                [-6.0,  5.0, -1.0,  1.0]
            ]
        };
        let mat_b = Matrix4x4 {
            data: [
                [ 8.0,  2.0,  2.0,  2.0],
                [ 3.0, -1.0,  7.0,  0.0],
                [ 7.0,  0.0,  5.0,  4.0],
                [ 6.0, -2.0,  0.0,  5.0]
            ]
        };
        let mat_c = mat_a * mat_b;

        assert_approx_4_by_4_eq!(mat_c * mat_b.inverse().unwrap(), mat_a);
    }

    #[test]
    fn multiply_by_a_tranformation_matrix() {
        let p = Point::new(-3.0, 4.0, 5.0);
        let transform = Matrix4x4::translation(5.0, -3.0, 2.0);

        assert_eq!(transform * p, Point::new(2.0, 1.0, 7.0))
    }

    #[test]
    fn multiply_by_a_the_inverse_of_a_tranformation_matrix() {
        let p = Point::new(-3.0, 4.0, 5.0);
        let transform = Matrix4x4::translation(5.0, -3.0, 2.0).inverse().unwrap();

        assert_eq!(transform * p, Point::new(-8.0, 7.0, 3.0))
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let v = Vector::new(-3.0, 4.0, 5.0);
        let transform = Matrix4x4::translation(5.0, -3.0, 2.0);

        assert_eq!(transform * v, v)
    }
}
