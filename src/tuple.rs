use std::ops;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point  {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        Vector {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude()
        }
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
}

impl ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vector { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl ops::Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Vector { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => ({
            let (a, b) = (&$a, &$b);
            assert!((*a - *b).abs() < 1.0e-6,
                    "{} is not approximately equal to {}", *a, *b);
        })
    }

    #[test]
    fn add_point_to_vector() {
        let p = Vector { x: 3.0, y: -2.0, z: 5.0 };
        let v = Point { x: -2.0, y: 3.0, z: 1.0  };

        assert_eq!(p + v, Point { x: 1.0, y: 1.0, z: 6.0 })
    }

    #[test]
    fn add_vector_to_point() {
        let p = Vector { x: 3.0, y: -2.0, z: 5.0 };
        let v = Point { x: -2.0, y: 3.0, z: 1.0  };

        assert_eq!(p + v, Point { x: 1.0, y: 1.0, z: 6.0 })
    }

    #[test]
    fn add_vector_to_vector() {
        let v1 = Vector { x: 3.0, y: -2.0, z: 5.0 };
        let v2 = Vector { x: -2.0, y: 3.0, z: 1.0  };

        assert_eq!(v1 + v2, Vector { x: 1.0, y: 1.0, z: 6.0 })
    }

    #[test]
    fn substract_two_points() {
        let p1 = Point { x: 3.0, y: 2.0, z: 1.0 };
        let p2 = Point { x: 5.0, y: 6.0, z: 7.0 };

        assert_eq!(p1 - p2, Vector { x: -2.0, y: -4.0, z: -6.0 })
    }

    #[test]
    fn substract_vector_from_point() {
        let p = Point { x: 3.0, y: 2.0, z: 1.0 };
        let v = Vector { x: 5.0, y: 6.0, z: 7.0 };

        assert_eq!(p - v, Point { x: -2.0, y: -4.0, z: -6.0 })
    }

    #[test]
    fn substract_two_vectors() {
        let p1 = Vector { x: 3.0, y: 2.0, z: 1.0 };
        let p2 = Vector { x: 5.0, y: 6.0, z: 7.0 };

        assert_eq!(p1 - p2, Vector { x: -2.0, y: -4.0, z: -6.0 })
    }

    #[test]
    fn negate_point() {
        let p  = Point { x: 1.0, y: -2.0, z: 3.0 };

        assert_eq!(-p, Point { x: -1.0, y: 2.0, z: -3.0 })
    }

    #[test]
    fn negate_vector() {
        let p  = Vector { x: 1.0, y: -2.0, z: 3.0 };

        assert_eq!(-p, Vector { x: -1.0, y: 2.0, z: -3.0 })
    }

    #[test]
    fn multiply_vector_with_float() {
        let p1  = Vector { x: 1.0, y: -2.0, z: 3.0 };
        let p2  = Vector { x: 1.0, y: -2.0, z: 3.0 };

        assert_eq!(p1 * 3.5, Vector { x: 3.5, y: -7.0, z: 10.5 });
        assert_eq!(p2 * 0.5, Vector { x: 0.5, y: -1.0, z: 1.5 });
    }

    #[test]
    fn divide_vector_with_float() {
        let p  = Vector { x: 1.0, y: -2.0, z: 3.0 };

        assert_eq!(p / 2.0, Vector { x: 0.5, y: -1.0, z: 1.5 });
    }

    #[test]
    fn magnitude_of_a_vector() {
        assert_approx_eq!(Vector { x: 1.0, y: 0.0, z: 0.0 }.magnitude(), 1.0);
        assert_approx_eq!(Vector { x: 0.0, y: 1.0, z: 0.0 }.magnitude(), 1.0);
        assert_approx_eq!(Vector { x: 0.0, y: 0.0, z: 1.0 }.magnitude(), 1.0);
        assert_approx_eq!(Vector { x: 1.0, y: 2.0, z: 3.0 }.magnitude(), 14.0_f32.sqrt());
        assert_approx_eq!(Vector { x: -1.0, y: -2.0, z: -3.0 }.magnitude(), 14.0_f32.sqrt());
    }

    #[test]
    fn normalize_a_vector() {
        assert_eq!(
            Vector { x: 4.0, y: 0.0, z: 0.0 }.normalize(),
            Vector { x: 1.0, y: 0.0, z: 0.0 }
        );
        assert_eq!(
            Vector { x: 1.0, y: 2.0, z: 3.0 }.normalize(),
            Vector { x: 1.0 / 14.0_f32.sqrt(), y: 2.0 / 14.0_f32.sqrt(), z: 3.0 / 14.0_f32.sqrt() }
        );
        assert_approx_eq!(Vector { x: 1.0, y: 2.0, z: 3.0 }.normalize().magnitude(), 1.0)
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let a = Vector { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vector { x: 2.0, y: 3.0, z: 4.0 };

        assert_approx_eq!(a.dot(&b), 20.0)
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Vector { x: 1.0, y: 2.0, z: 3.0 };
        let b = Vector { x: 2.0, y: 3.0, z: 4.0 };

        assert_eq!(a.cross(&b), Vector { x: -1.0, y: 2.0, z: -1.0 });
        assert_eq!(b.cross(&a), Vector { x: 1.0, y: -2.0, z: 1.0 });
    }
}
