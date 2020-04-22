#[cfg(test)]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {{
        let (a, b) = (&$a, &$b);
        assert!(
            (*a - *b).abs() < 1.0e-5,
            "{} is not approximately equal to {}",
            *a,
            *b
        );
    }};
}

#[cfg(test)]
macro_rules! assert_approx_4_by_4_eq {
    ($a:expr, $b:expr) => {{
        let (a, b) = (&$a, &$b);
        for row in 0..=3 {
            for col in 0..=3 {
                assert_approx_eq!(a[col][row], b[col][row]);
            }
        }
    }};
}

mod matrix;
mod point;
mod tuple;
mod vector;

pub use self::matrix::{Matrix2x2, Matrix3x3, Matrix4x4};
pub use self::point::Point;
pub use self::tuple::Tuple;
pub use self::vector::Vector;
