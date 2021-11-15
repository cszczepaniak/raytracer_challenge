extern crate raytracer;

use num_traits::Float;
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

struct Environment<T>
where
    T: Float,
{
    // the acceleration of gravity
    gravity: Vector<T>,
    // the acceleration of the wind
    wind: Vector<T>,
}

#[derive(Debug)]
struct Particle<T>
where
    T: Float,
{
    position: Vector<T>,
    velocity: Vector<T>,
    acceleration: Vector<T>,
}

impl<T> Particle<T>
where
    T: Float,
{
    fn with_env(env: Environment<T>, initial_pos: Vector<T>, initial_vel: Vector<T>) -> Self {
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
