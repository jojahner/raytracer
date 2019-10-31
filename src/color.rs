use std::ops;

const EPSILON: f32 = 10e-6;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }
}

impl ops::Add<Color> for Color {
    type Output = Self;

    fn add(self, other: Color) -> Self::Output {
        Color::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl ops::Sub<Color> for Color {
    type Output = Self;

    fn sub(self, other: Color) -> Self::Output {
        Color::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Color::new(self.red * other, self.green * other, self.blue * other)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self::Output {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        (self.red - other.red).abs() <= EPSILON
            && (self.green - other.green).abs() <= EPSILON
            && (self.blue - other.blue).abs() <= EPSILON
    }
}

#[cfg(test)]
mod test {
    use super::Color;

    #[test]
    fn adding_two_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn substracting_two_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiply_color_with_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8))
    }

    #[test]
    fn multiply_two_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04))
    }
}
