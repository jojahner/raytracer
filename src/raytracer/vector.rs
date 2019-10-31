use std::ops;

use crate::raytracer::point::Point;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        Vector::new(
            self.x / self.magnitude(),
            self.y / self.magnitude(),
            self.z / self.magnitude(),
        )
    }

    pub fn dot(&self, other: Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }
}

impl ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => {{
            let (a, b) = (&$a, &$b);
            assert!(
                (*a - *b).abs() < 1.0e-6,
                "{} is not approximately equal to {}",
                *a,
                *b
            );
        }};
    }

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
        assert_approx_eq!(Vector::new(1.0, 2.0, 3.0).magnitude(), 14.0_f32.sqrt());
        assert_approx_eq!(Vector::new(1.0, 2.0, 3.0).magnitude(), 14.0_f32.sqrt());
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
                1.0 / 14.0_f32.sqrt(),
                2.0 / 14.0_f32.sqrt(),
                3.0 / 14.0_f32.sqrt()
            )
        );
        assert_approx_eq!(Vector::new(1.0, 2.0, 3.0).normalize().magnitude(), 1.0)
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        assert_approx_eq!(a.dot(b), 20.0)
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(a.cross(b), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(a), Vector::new(1.0, -2.0, 1.0));
    }
}
