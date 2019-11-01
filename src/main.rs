use raytracer::math::{Point, Vector};
use raytracer::{Canvas, Color};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(projectile: Projectile, environment: Environment) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + environment.gravity + environment.wind;

    Projectile { position, velocity }
}

fn main() {
    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let environment = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);

    loop {
        let x_pos = projectile.position.x as usize;
        let y_pos = 550 - (projectile.position.y as usize);
        canvas.write_pixel(x_pos, y_pos, Color::new(1.0, 0.0, 0.0));

        projectile = tick(projectile, environment);

        if projectile.position.y < 0.0 {
            break;
        }
    }
    let mut data = canvas.to_tga();

    let mut file = match File::create("output.tga") {
        Err(why) => panic!("couldn't create output.tga: {}", why.description()),
        Ok(file) => file,
    };

    match file.write_all(data.as_mut()) {
        Err(why) => panic!("couldn't write to output.tga: {}", why.description()),
        Ok(_) => println!("successfully wrote to output.tga"),
    }
}
