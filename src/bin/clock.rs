use std::{f64::consts::PI, fs};

use raytracer::{
    canvas::{to_png::ToPng, to_ppm::ToPpm, Canvas},
    color::Color,
    matrix::{Matrix, Rotation},
    point::Point,
};

pub fn pixel_from_point(point: &Point, canvas: &Canvas) -> Option<(usize, usize)> {
    let x = point[0];
    let y = point[1];
    if x < 0.0 || y < 0.0 {
        return None;
    }
    if x.round() as usize >= canvas.width || y.round() as usize >= canvas.height {
        return None;
    }

    Some((x.round() as usize, canvas.height - 1 - y.round() as usize))
}

fn main() {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 400;
    const R: usize = 150;

    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let pt = Point::new(R as f64, 0.0, 0.0);
    let origin = Point::new((WIDTH / 2) as f64, (HEIGHT / 2) as f64, 0.0);
    let color = Color::new(1.0, 0.0, 0.0);
    let translation = Matrix::translate(origin[0], origin[1], 0.0);

    for i in 0..12 {
        let rotation = Matrix::rotate(Rotation::Z, (2.0 * PI * (i as f64)) / 12.0);
        let rotated = translation * rotation * pt;
        let px = pixel_from_point(&rotated, &canvas);
        let x;
        let y;
        match px {
            Some((cx, cy)) => {
                x = cx;
                y = cy;
            }
            None => {
                println!("out of bounds");
                break;
            }
        };
        canvas.write_pixel(x, y, color);

        // Make lines
        let len = match i % 3 {
            0 => 10,
            _ => 5,
        };
        for i in 1..=len {
            // ray is a vector representing the segment from the origin to the point on the clock
            let ray = rotated - origin;
            // scaled is the ray, but scaled down towards the origin by 1 pixel
            let scaled: Point = (ray.normalize() * (ray.magnitude() - i as f64)).into();

            let l_px = pixel_from_point(&(translation * scaled), &canvas);
            if let Some((x, y)) = l_px {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    println!("Saving to PPM...");
    fs::write("output.ppm", canvas.to_ppm()).expect("error writing to file");

    println!("Saving to PNG...");
    let f = fs::File::create("output.png").expect("error creating 'output.png'");
    canvas.to_png(f).expect("error writing file data");
}
