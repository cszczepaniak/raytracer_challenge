use std::{
    f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4},
    fs,
    sync::Mutex,
};

use itertools::Itertools;
use rayon::prelude::*;
use raytracer::{
    camera::Camera,
    canvas::{Canvas, ToPng},
    color::Color,
    light::PointLight,
    material::Phong,
    matrix::{Matrix, Rotation},
    point::Point,
    sphere::Sphere,
    vector::Vector,
    world::World,
};

fn main() {
    let canvas_width = 3840;
    let canvas_height = 2160;

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    // Floor and walls. Cheat by using squashed spheres...
    let floor_and_wall_material = Phong {
        color: Color::new(0.5, 0.45, 0.45),
        specular: 0.0,
        ..Phong::default()
    }
    .into();

    let floor_sphere = Sphere::default()
        .with_material(floor_and_wall_material)
        .with_transform(Matrix::scale(10.0, 0.01, 10.0));

    let left_wall_sphere = Sphere::default()
        .with_material(floor_and_wall_material)
        .with_transform(
            Matrix::translate(0.0, 0.0, 5.0)
                * Matrix::rotate(Rotation::Y, -FRAC_PI_4)
                * Matrix::rotate(Rotation::X, FRAC_PI_2)
                * Matrix::scale(10.0, 0.01, 10.0),
        );

    let right_wall_sphere = Sphere::default()
        .with_material(floor_and_wall_material)
        .with_transform(
            Matrix::translate(0.0, 0.0, 5.0)
                * Matrix::rotate(Rotation::Y, FRAC_PI_4)
                * Matrix::rotate(Rotation::X, FRAC_PI_2)
                * Matrix::scale(10.0, 0.01, 10.0),
        );

    let left_material = Phong {
        color: Color::new(1.0, 0.8, 0.1),
        diffuse: 0.7,
        specular: 0.3,
        ..Phong::default()
    }
    .into();
    let left_sphere = Sphere::default()
        .with_material(left_material)
        .with_transform(Matrix::translate(-1.5, 0.33, -0.75) * Matrix::scale(0.33, 0.33, 0.33));

    let middle_material = Phong {
        color: Color::new(0.1, 1.0, 0.5),
        diffuse: 0.7,
        specular: 0.3,
        ..Phong::default()
    }
    .into();
    let middle_sphere = Sphere::default()
        .with_material(middle_material)
        .with_transform(Matrix::translate(-0.5, 1.0, 0.5));

    let right_material = Phong {
        color: Color::new(0.5, 1.0, 0.1),
        diffuse: 0.7,
        specular: 0.3,
        ..Phong::default()
    }
    .into();
    let right_sphere = Sphere::default()
        .with_material(right_material)
        .with_transform(Matrix::translate(1.5, 0.5, -0.5) * Matrix::scale(0.5, 0.5, 0.5));

    let world = World::new(
        vec![
            floor_sphere.into(),
            left_wall_sphere.into(),
            right_wall_sphere.into(),
            middle_sphere.into(),
            left_sphere.into(),
            right_sphere.into(),
        ],
        vec![light],
    );

    let camera = Camera::new(canvas_width, canvas_height, FRAC_PI_3).look_at_from_position(
        Point::new(0.0, 2.5, -5.0),
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
