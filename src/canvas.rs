use crate::color::Color;

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
        assert!(x < self.width); // TODO: real error handling
        assert!(y < self.height);

        self.bitmap[(y * self.width) + x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        assert!(x < self.width); // TODO: real error handling
        assert!(y < self.height);

        self.bitmap[(y * self.width) + x]
    }

    pub fn to_tga(&self) -> Vec<u8> {
        let mut tga: [u8; 18] = [0; 18];

        // tga header
        // see: http://www.gamers.org/dEngine/quake3/TGA.txt
        tga[2] = 2; // uncompressed RGB data
        tga[12] = (255 & self.width) as u8;
        tga[13] = (255 & (self.width >> 8)) as u8;
        tga[14] = (255 & self.height) as u8;
        tga[15] = (255 & (self.height >> 8)) as u8;
        tga[16] = 24; // 24 bits per pixel
        tga[17] = 0b0010_0000; // screen origin upper left-hand corner

        let mut image: Vec<u8> = tga.to_vec();

        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.pixel_at(x, y);

                image.push(Color::convert_component(color.blue));
                image.push(Color::convert_component(color.green));
                image.push(Color::convert_component(color.red));
            }
        }

        image
    }
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
}
