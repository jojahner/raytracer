use std::ops;

use crate::raytracer::vector::Vector;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }
}

impl ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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
