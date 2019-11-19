use std::convert::From;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::math::Point;
use crate::math::Vector;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }
}

impl Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Self::Output {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Self::Output {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Tuple::new(
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other,
        )
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Tuple::new(
            self.x / other,
            self.y / other,
            self.z / other,
            self.w / other,
        )
    }
}

impl From<Vector> for Tuple {
    fn from(vector: Vector) -> Self {
        Tuple::new(vector.x, vector.y, vector.z, 0.0)
    }
}

impl From<Point> for Tuple {
    fn from(point: Point) -> Self {
        Tuple::new(point.x, point.y, point.z, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_tuple_to_tuple() {
        let t1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let t2 = Tuple::new(-2.0, 3.0, 1.0, 1.0);

        assert_eq!(t1 + t2, Tuple::new(1.0, 1.0, 6.0, 2.0))
    }

    #[test]
    fn substract_two_tuples() {
        let t1 = Tuple::new(3.0, 2.0, 1.0, 2.0);
        let t2 = Tuple::new(5.0, 6.0, 7.0, 3.0);

        assert_eq!(t1 - t2, Tuple::new(-2.0, -4.0, -6.0, -1.0))
    }

    #[test]
    fn negate_tuple() {
        let t = Tuple::new(1.0, -2.0, 3.0, 2.0);

        assert_eq!(-t, Tuple::new(-1.0, 2.0, -3.0, -2.0))
    }

    #[test]
    fn multiply_tuple_with_float() {
        let t1 = Tuple::new(1.0, -2.0, 3.0, 2.0);
        //   let t2 = Tuple::new(1.0, -2.0, 3.0, 2.0);

        assert_eq!(t1 * 3.5, Tuple::new(3.5, -7.0, 10.5, 7.0));
        assert_eq!(t1 * 0.5, Tuple::new(0.5, -1.0, 1.5, 1.0));
    }

    #[test]
    fn divide_tuple_with_float() {
        let t = Tuple::new(1.0, -2.0, 3.0, 2.0);

        assert_eq!(t / 2.0, Tuple::new(0.5, -1.0, 1.5, 1.0));
    }
}
