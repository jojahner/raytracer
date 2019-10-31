mod tuple;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Projectile {
    position: tuple::Point,
    velocity: tuple::Vector
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Environment {
    gravity: tuple::Vector,
    wind: tuple::Vector
}

fn tick(projectile: Projectile, environment: Environment) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + environment.gravity + environment.wind;

    Projectile { position, velocity }
}


fn main() {
    let mut projectile = Projectile {
        position: tuple::Point { x: 0.0, y: 1.0, z: 0.0 },
        velocity: tuple::Vector { x: 0.9, y: 0.75, z: 0.0 }.normalize()
    };

    let environment = Environment {
        gravity: tuple::Vector { x: 0.0, y: -0.1, z: 0.0 },
        wind: tuple::Vector { x: -0.01, y: 0.0, z: 0.0 },
    };

    loop {
        println!("{:?}, {:?}", projectile.position.x, projectile.position.y);
        projectile = tick(projectile, environment);

        if projectile.position.y < 0.0 {
            break;
        }
    }
}
