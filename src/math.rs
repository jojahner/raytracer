macro_rules! define_tuple3 {
    ($T:ident) => {
        #[derive(Copy, Clone, PartialEq, Debug)]
        pub struct $T {
            pub x: f32,
            pub y: f32,
            pub z: f32
        }
    }
}

macro_rules! define_tuple3_constructor {
    ($T:ident) => {
        impl $T {
            pub fn new(x: f32, y: f32, z: f32) -> $T {
                $T { x, y, z }
            }
        }
    };
}

macro_rules! define_tuple3_add {
    ($T:ident, $Other:ident, $Output:ident) => {
        impl ops::Add<$Other> for $T {
            type Output = $Output;

            fn add(self, other: $Other) -> Self::Output {
                $Output::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }
    };
}

macro_rules! define_tuple3_sub {
    ($T:ident, $Other:ident, $Output:ident) => {
        impl ops::Sub<$Other> for $T {
            type Output = $Output;

            fn sub(self, other: $Other) -> Self::Output {
                $Output::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }
    };
}

macro_rules! define_tuple3_neg {
    ($T:ident) => {
        impl ops::Neg for $T {
            type Output = $T;

            fn neg(self) -> Self::Output {
                $T::new(-self.x, -self.y, -self.z)
            }
        }
    };
}

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

mod point;
mod vector;

pub use self::point::Point;
pub use self::vector::Vector;
