extern crate raytracer;

use raytracer::tuples::vector::Vector;

fn main() {
    let env = Environment {
        gravity: Vector::new(0.0, -0.2, 0.0),
        wind: Vector::new(-0.02, 0.0, 0.0),
    };
    let mut particle =
        Particle::with_env(env, Vector::new(0.0, 2.0, 0.0), Vector::new(2.0, 0.0, 0.0));
    while particle.position[1] >= 0.0 {
        println!("{:?}", particle);
        particle.step();
    }
    println!("{:?}", particle);
}

struct Environment {
    // the acceleration of gravity
    gravity: Vector,
    // the acceleration of the wind
    wind: Vector,
}

#[derive(Debug)]
struct Particle {
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
}

impl Particle {
    fn with_env(env: Environment, initial_pos: Vector, initial_vel: Vector) -> Self {
        Self {
            position: initial_pos,
            velocity: initial_vel,
            acceleration: env.gravity + env.wind,
        }
    }

    fn step(&mut self) {
        self.velocity = self.velocity + self.acceleration;
        self.position = self.position + self.velocity;
    }
}
