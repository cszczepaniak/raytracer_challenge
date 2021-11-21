use std::{f64::consts::PI, fs};

use raytracer::{
    canvas::{to_png::ToPng, to_ppm::ToPpm, Canvas},
    color::Color,
    matrix::{Matrix, Rotation},
    point::Point,
    vector::Vector,
};

#[derive(Debug)]
enum Pixel {
    Coordinate { x: usize, y: usize },
    OutOfBounds,
}

impl Pixel {
    pub fn from_point_for_canvas(
        point: &Point<f64>,
        origin: &Point<f64>,
        canvas: &Canvas<f64>,
    ) -> Pixel {
        let x = point[0] + origin[0];
        let y = point[1] + origin[1];
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
    let mut canvas = Canvas::new(400, 400);
    let pt = Point::new(100.0, 0.0, 0.0);
    let origin = Point::new(200.0, 200.0, 0.0);
    let color = Color::new(1.0, 0.0, 0.0);

    for i in 0..12 {
        let transform = Matrix::rotate(Rotation::Z, (2.0 * PI * (i as f64)) / 12.0);
        let rotated = transform * pt;
        let px = Pixel::from_point_for_canvas(&rotated, &origin, &canvas);
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
        let v: Vector<_> = rotated.into();
        let len = match i % 3 {
            0 => 10,
            _ => 5,
        };
        for i in 1..=len {
            let scaled = v.normalize() * (v.magnitude() - i as f64);
            let l_px = Pixel::from_point_for_canvas(&scaled.into(), &origin, &canvas);
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
