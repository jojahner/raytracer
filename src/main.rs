use raytracer::math::{Point, Matrix4x4};
use raytracer::{Canvas, Color};

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut canvas = Canvas::new(500, 500);

    let start = Point::new(0.0, 0.0, 1.0);
    let radius = 200.0;
    for hour in 1..=60 {
        let rotation = Matrix4x4::rotation_y(hour as f64 * (std::f64::consts::PI / 30.0));
        let point = rotation * start;

        let x_pos = (point.x * radius + 250.0) as usize;
        let y_pos = (point.z * radius + 250.0) as usize;
        if hour % 5 == 0 {
            canvas.write_pixel(x_pos, y_pos, Color::new(1.0, 0.0, 0.0));
        } else {
            canvas.write_pixel(x_pos, y_pos, Color::new(1.0, 1.0, 1.0));
        }
    }

    canvas.write_pixel(250, 250, Color::new(0.0, 1.0, 1.0));

    let mut data = canvas.to_tga();

    let mut file = match File::create("output.tga") {
        Err(why) => panic!("couldn't create output.tga: {}", why.to_string()),
        Ok(file) => file,
    };

    match file.write_all(data.as_mut()) {
        Err(why) => panic!("couldn't write to output.tga: {}", why.to_string()),
        Ok(_) => println!("successfully wrote to output.tga"),
    }
}
