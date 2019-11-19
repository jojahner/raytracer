use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::Point;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        Vector::new(
            self.x / self.magnitude(),
            self.y / self.magnitude(),
            self.z / self.magnitude(),
        )
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Self::Output {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Vector::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Vector::new(self.x / other, self.y / other, self.z / other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_point_to_vector() {
        let v = Vector::new(3.0, -2.0, 5.0);
        let p = Point::new(-2.0, 3.0, 1.0);

        assert_eq!(v + p, Point::new(1.0, 1.0, 6.0))
    }

    #[test]
    fn add_vector_to_vector() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);

        assert_eq!(v1 + v2, Vector::new(1.0, 1.0, 6.0))
    }

    #[test]
    fn substract_two_vectors() {
        let p1 = Vector::new(3.0, 2.0, 1.0);
        let p2 = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0))
    }

    #[test]
    fn negate_vector() {
        let p = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(-p, Vector::new(-1.0, 2.0, -3.0))
    }

    #[test]
    fn multiply_vector_with_float() {
        let p1 = Vector::new(1.0, -2.0, 3.0);
        let p2 = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(p1 * 3.5, Vector::new(3.5, -7.0, 10.5));
        assert_eq!(p2 * 0.5, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn divide_vector_with_float() {
        let p = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(p / 2.0, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn magnitude_of_a_vector() {
        assert_approx_eq!(Vector::new(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_approx_eq!(Vector::new(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_approx_eq!(Vector::new(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_approx_eq!(Vector::new(1.0, 2.0, 3.0).magnitude(), 14.0_f64.sqrt());
        assert_approx_eq!(Vector::new(1.0, 2.0, 3.0).magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalize_a_vector() {
        assert_eq!(
            Vector::new(4.0, 0.0, 0.0).normalize(),
            Vector::new(1.0, 0.0, 0.0)
        );
        assert_eq!(
            Vector::new(1.0, 2.0, 3.0).normalize(),
            Vector::new(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )
        );
        assert_approx_eq!(Vector::new(1.0, 2.0, 3.0).normalize().magnitude(), 1.0)
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        assert_approx_eq!(a.dot(&b), 20.0)
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(a.cross(b), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(a), Vector::new(1.0, -2.0, 1.0));
    }
}
