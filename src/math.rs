#[cfg(test)]
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

mod matrix;
mod point;
mod tuple;
mod vector;

pub use self::matrix::{Matrix2x2, Matrix3x3, Matrix4x4};
pub use self::point::Point;
pub use self::tuple::Tuple;
pub use self::vector::Vector;
