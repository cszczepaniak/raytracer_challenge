use std::{fs, sync::Mutex};

use itertools::Itertools;
use rayon::prelude::*;
use raytracer::{
    canvas::{Canvas, ToPng},
    color::Color,
    intersection::Intersectable,
    matrix::Matrix,
    point::Point,
    ray::Ray,
    sphere::Sphere,
};

fn main() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 5.0;
    let wall_size = 10.0;

    let canvas_size = 4096;
    let pixel_world_ratio = wall_size / canvas_size as f64;

    let color = Color::new(0.5, 0.2, 0.1);
    let sphere: Sphere = Sphere::default().with_transform(Matrix::identity());

    let canvas_mutex = Mutex::new(Canvas::new(canvas_size, canvas_size));

    let progress = indicatif::ProgressBar::new((canvas_size * canvas_size) as u64);
    progress.set_draw_rate(5);

    (0..canvas_size)
        .cartesian_product(0..canvas_size)
        .par_bridge()
        .for_each(|(row, col)| {
            let target_point = Point::new(
                (col as f64 * pixel_world_ratio) - wall_size / 2.0,
                (row as f64 * pixel_world_ratio) - wall_size / 2.0,
                wall_z,
            );
            let ray = Ray::new(origin, (target_point - origin).normalize());

            let intersections = sphere.intersect(ray);
            if intersections.hit().is_some() {
                let mut canvas = canvas_mutex.lock().unwrap();
                canvas.write_pixel(col, row, color);
            }
            progress.inc(1);
        });

    progress.finish();

    println!("Saving to PNG...");
    let f = fs::File::create("output.png").expect("error creating 'output.png'");

    let canvas = canvas_mutex.lock().unwrap();
    canvas.to_png(f).expect("error writing file data");
}
