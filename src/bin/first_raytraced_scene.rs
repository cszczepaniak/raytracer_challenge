use std::fs;

use raytracer::{
    canvas::{to_png::ToPng, Canvas},
    color::Color,
    intersection::Intersectable,
    point::Point,
    ray::Ray,
    sphere::Sphere,
};

fn main() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 5.0;
    let wall_size = 10.0;

    let canvas_size = 512;
    let pixel_world_ratio = wall_size / canvas_size as f64;

    let color = Color::new(0.5, 0.2, 0.1);
    let mut canvas = Canvas::new(canvas_size, canvas_size);

    let sphere = Sphere::default();

    for row in 0..canvas_size {
        for col in 0..canvas_size {
            let target_point = Point::new(
                (col as f64 * pixel_world_ratio) - wall_size / 2.0,
                (row as f64 * pixel_world_ratio) - wall_size / 2.0,
                wall_z,
            );
            let ray = Ray::new(origin, (target_point - origin).normalize());

            let intersections = sphere.intersect(ray);
            if intersections.hit().is_some() {
                canvas.write_pixel(col, row, color);
            }
        }
    }

    println!("Saving to PNG...");
    let f = fs::File::create("output.png").expect("error creating 'output.png'");
    canvas.to_png(f).expect("error writing file data");
}
