use std::{f64::consts::FRAC_PI_3, fs, sync::Mutex};

use itertools::Itertools;
use rayon::prelude::*;
use raytracer::{
    camera::Camera,
    canvas::{Canvas, ToPng},
    color::Color,
    light::PointLight,
    material::{Material, Phong, PhongAttribute},
    matrix::Matrix,
    point::Point,
    sphere::Sphere,
    vector::Vector,
    world::World,
};

fn main() {
    let canvas_width = 7680;
    let canvas_height = 4320;

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let left_material = Material::Phong(Phong::new(&[
        PhongAttribute::Color(Color::new(1.0, 0.8, 0.1)),
        PhongAttribute::Diffuse(0.7),
        PhongAttribute::Specular(0.3),
    ]));
    let left_sphere = Sphere::default()
        .with_material(left_material)
        .with_transform(Matrix::translate(-1.5, 0.33, -0.75) * Matrix::scale(0.33, 0.33, 0.33));

    let middle_material = Material::Phong(Phong::new(&[
        PhongAttribute::Color(Color::new(0.1, 1.0, 0.5)),
        PhongAttribute::Diffuse(0.7),
        PhongAttribute::Specular(0.3),
    ]));
    let middle_sphere = Sphere::default()
        .with_material(middle_material)
        .with_transform(Matrix::translate(-0.5, 1.0, 0.5));

    let right_material = Material::Phong(Phong::new(&[
        PhongAttribute::Color(Color::new(0.5, 1.0, 0.1)),
        PhongAttribute::Diffuse(0.7),
        PhongAttribute::Specular(0.3),
    ]));
    let right_sphere = Sphere::default()
        .with_material(right_material)
        .with_transform(Matrix::translate(1.5, 0.5, -0.5) * Matrix::scale(0.5, 0.5, 0.5));

    let world = World::new(
        vec![
            left_sphere.into(),
            middle_sphere.into(),
            right_sphere.into(),
        ],
        vec![light],
    );

    let camera = Camera::new(canvas_width, canvas_height, FRAC_PI_3).look_at_from_position(
        Point::new(0.0, 1.0, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas_mutex = Mutex::new(Canvas::new(canvas_width, canvas_height));

    let progress = indicatif::ProgressBar::new((canvas_width * canvas_height) as u64);
    progress.set_draw_rate(5);

    (0..canvas_height)
        .cartesian_product(0..canvas_width)
        .par_bridge()
        .for_each(|(row, col)| {
            let color = world.color_at(camera.ray_for_pixel(col, row));
            let mut canvas = canvas_mutex.lock().unwrap();
            canvas.write_pixel(col, row, color);
            progress.inc(1);
        });

    progress.finish();

    println!("Saving to PNG...");
    let f = fs::File::create("output.png").expect("error creating 'output.png'");

    let canvas = canvas_mutex.lock().unwrap();
    canvas.to_png(f).expect("error writing file data");
}
