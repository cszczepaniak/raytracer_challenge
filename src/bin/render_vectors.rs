extern crate raytracer;

use std::fs;

use raytracer::{
    canvas::{self, to_ppm::ToPPM, Rectangle},
    {color::Color, vector::Vector},
};

fn main() {
    let env = Environment {
        gravity: Vector::new(0.0, -0.2, 0.0),
        wind: Vector::new(-0.02, 0.0, 0.0),
    };
    let mut particle = Particle::with_env(
        env,
        Default::default(),
        Vector::new(2.0, 4.0, 0.0).normalize() * 14.5,
    );
    let mut canvas = canvas::Canvas::new(700, 450);
    while particle.position[1] >= 0.0 {
        println!("{:?}", particle.position);
        if let Some((x, y)) = particle.pos_in_canvas(&canvas) {
            canvas.write_pixel(x, y, Color::new(1.0, 0.0, 0.0));
        }
        particle.step();
    }
    println!("{:?}", particle.position);

    println!("Saving to file...");

    fs::write("output.ppm", canvas.to_ppm()).expect("error writing to file");
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

    fn pos_in_canvas<T>(&self, canvas: &T) -> Option<(usize, usize)>
    where
        T: Rectangle,
    {
        if self.position[0] < 0.0 || self.position[1] < 0.0 {
            return None;
        }
        if self.position[0].round() as usize >= canvas.width()
            || self.position[1].round() as usize >= canvas.height()
        {
            return None;
        }
        Some((
            self.position[0].round() as usize,
            canvas.height() - 1 - self.position[1].round() as usize,
        ))
    }
}
