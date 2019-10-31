use crate::color::Color;
use std::fmt::Write;

pub struct Canvas {
    width: usize,
    height: usize,
    bitmap: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            bitmap: vec![Color::black(); width * height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.bitmap[(y * self.width) + x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        assert!(x < self.width);
        assert!(y < self.height);

        self.bitmap[(y * self.width) + x]
    }

    pub fn to_ppm(&self) -> String {
        let mut output = String::new();
        write_ppm_header(&mut output, self);

        for y in 0..self.height {
            for x in 0..self.width {
                write_pixel_to_ppm(&mut output, self.pixel_at(x, y));

                if x % 5 == 4 {
                    output.write_str("\n").unwrap();
                } else {
                    output.write_str(" ").unwrap();
                }
            }
        }

        output.write_str("\n").unwrap();
        output
    }
}

fn write_ppm_header(w: &mut Write, canvas: &Canvas) {
    w.write_str("P3\n").unwrap();
    w.write_fmt(format_args!("{} {}\n", canvas.width, canvas.height))
        .unwrap();
    w.write_str("255\n").unwrap();
}

fn write_pixel_to_ppm(w: &mut Write, color: Color) {
    let red = Color::convert_component(color.red);
    let green = Color::convert_component(color.green);
    let blue = Color::convert_component(color.blue);

    w.write_fmt(format_args!("{} {} {}", red, green, blue))
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::Canvas;
    use crate::color::Color;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.bitmap.len(), 200);
        assert_eq!(c.bitmap[0], Color::black());
    }

    #[test]
    fn writing_pixel_to_canvas() {
        let mut c = Canvas::new(2, 3);
        let red = Color::new(1.0, 0.0, 0.0);

        c.write_pixel(1, 2, red);
        assert_eq!(c.bitmap[5], Color::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn reading_pixel_from_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.write_pixel(5, 5, red);
        assert_eq!(c.pixel_at(5, 5), Color::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn write_a_ppm_file() {
        let mut c = Canvas::new(10, 2);
        c.write_pixel(0, 0, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(1, 0, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(2, 0, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(3, 0, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(4, 0, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(5, 0, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(6, 0, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(7, 0, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(8, 0, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(9, 0, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(0, 1, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(1, 1, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(2, 1, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(3, 1, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(4, 1, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(5, 1, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(6, 1, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(7, 1, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(8, 1, Color::new(1.0, 0.8, 0.6));
        c.write_pixel(9, 1, Color::new(1.0, 0.8, 0.6));

        let ppm = c.to_ppm();
        let mut lines = ppm.lines();

        assert_eq!(lines.next(), Some("P3"));
        assert_eq!(lines.next(), Some("10 2"));
        assert_eq!(lines.next(), Some("255"));

        assert_eq!(
            lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153")
        );
        assert_eq!(
            lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153")
        );
        assert_eq!(
            lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153")
        );
        assert_eq!(
            lines.next(),
            Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153")
        );
        assert_eq!(lines.next(), Some(""));
    }
}
