extern crate raytracer;

use raytracer::vector::Vector;

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
    gravity: Vector<f64>,
    // the acceleration of the wind
    wind: Vector<f64>,
}

#[derive(Debug)]
struct Particle {
    position: Vector<f64>,
    velocity: Vector<f64>,
    acceleration: Vector<f64>,
}

impl Particle {
    fn with_env(env: Environment, initial_pos: Vector<f64>, initial_vel: Vector<f64>) -> Self {
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
