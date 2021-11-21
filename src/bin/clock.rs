use std::{f64::consts::PI, fs};

use raytracer::{
    canvas::{to_png::ToPng, to_ppm::ToPpm, Canvas},
    color::Color,
    matrix::{Matrix, Rotation},
    point::Point,
};

#[derive(Debug)]
enum Pixel {
    Coordinate { x: usize, y: usize },
    OutOfBounds,
}

impl Pixel {
    pub fn from_point_for_canvas(point: &Point<f64>, canvas: &Canvas<f64>) -> Pixel {
        let x = point[0];
        let y = point[1];
        if x < 0.0 || y < 0.0 {
            return Pixel::OutOfBounds;
        }
        if x.round() as usize >= canvas.width || y.round() as usize >= canvas.height {
            return Pixel::OutOfBounds;
        }

        Pixel::Coordinate {
            x: x.round() as usize,
            y: canvas.height - 1 - y.round() as usize,
        }
    }
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
        let px = Pixel::from_point_for_canvas(&rotated, &canvas);
        let x;
        let y;
        match px {
            Pixel::Coordinate { x: cx, y: cy } => {
                x = cx;
                y = cy;
            }
            Pixel::OutOfBounds => {
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
            let scaled: Point<_> = (ray.normalize() * (ray.magnitude() - i as f64)).into();

            let l_px = Pixel::from_point_for_canvas(&(translation * scaled), &canvas);
            if let Pixel::Coordinate { x, y } = l_px {
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
