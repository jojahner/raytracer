mod raytracer;
use raytracer::{Point, Vector};

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
        velocity: Vector::new(0.9, 0.75, 0.0).normalize(),
    };

    let environment = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    loop {
        println!("{:?}, {:?}", projectile.position.x, projectile.position.y);
        projectile = tick(projectile, environment);

        if projectile.position.y < 0.0 {
            break;
        }
    }
}
