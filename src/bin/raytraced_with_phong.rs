use std::{fs, sync::Mutex};

use itertools::Itertools;
use rayon::prelude::*;
use raytracer::{
    canvas::{to_png::ToPng, Canvas},
    color::Color,
    intersection::Intersectable,
    light::PointLight,
    material::{Illuminated, Phong, PhongAttribute},
    point::Point,
    ray::Ray,
    sphere::Sphere,
};

fn main() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 11.0;
    let wall_size = 10.0;

    let canvas_size = 2048;
    let pixel_world_ratio = wall_size / canvas_size as f64;

    let material = Phong::new(&[PhongAttribute::Color(Color::new(1.0, 0.75, 0.0))]);
    let sphere: Sphere = Sphere::default().with_material(material);

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let canvas_mutex = Mutex::new(Canvas::new(canvas_size, canvas_size));

    let progress = indicatif::ProgressBar::new((canvas_size * canvas_size) as u64);
    progress.set_draw_rate(5);

    (0..canvas_size)
        .cartesian_product(0..canvas_size)
        .par_bridge()
        .for_each(|(row, col)| {
            let target_point = Point::new(
                (col as f64 * pixel_world_ratio) - wall_size / 2.0,
                -(row as f64 * pixel_world_ratio) + wall_size / 2.0,
                wall_z,
            );
            let ray = Ray::new(origin, (target_point - origin).normalize());

            let intersections = sphere.intersect(ray);
            let hit = intersections.hit();
            if let Some(hit) = hit {
                let position = ray.position(hit.t);
                let normal = hit.body.normal_at(position);
                let eye = -ray.direction;
                let color = hit.body.material.lighting(&light, position, eye, normal);

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