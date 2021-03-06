use std::ops;

const EPSILON: f64 = 10e-6;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn convert_component(component: f64) -> u8 {
        let value = (component * 255.0).round() as i32;
        match value {
            val if val < 0 => 0,
            val if val > 255 => 255,
            val => val as u8,
        }
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

impl ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
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
