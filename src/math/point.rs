use std::ops;

use crate::math::Vector;

define_tuple3!(Point);
define_tuple3_constructor!(Point);
define_tuple3_add!(Point, Vector, Point);
define_tuple3_sub!(Point, Point, Vector);
define_tuple3_sub!(Point, Vector, Point);
define_tuple3_neg!(Point);

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
