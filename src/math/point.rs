use std::ops;

use crate::math::Vector;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < 1.0e-6
            && (self.y - other.y).abs() < 1.0e-6
            && (self.z - other.z).abs() < 1.0e-6
    }
}

impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, other: Vector) -> Self::Output {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Self::Output {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, other: Vector) -> Self::Output {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vector_to_point() {
        let v = Vector::new(3.0, -2.0, 5.0);
        let p = Point::new(-2.0, 3.0, 1.0);

        assert_eq!(p + v, Point::new(1.0, 1.0, 6.0))
    }

    #[test]
    fn substract_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0))
    }

    #[test]
    fn substract_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0))
    }

    #[test]
    fn negate_point() {
        let p = Point::new(1.0, -2.0, 3.0);

        assert_eq!(-p, Point::new(-1.0, 2.0, -3.0))
    }
}
