use std::{f64::consts::PI, fs};

use raytracer::{
    canvas::{to_png::ToPng, to_ppm::ToPpm, Canvas},
    color::Color,
    matrix::Matrix,
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
        mut point: Point<f64>,
        origin: &Point<f64>,
        canvas: &Canvas<f64>,
    ) -> Pixel {
        point[0] += origin[0];
        point[1] += origin[1];
        if point[0] < 0.0 || point[1] < 0.0 {
            return Pixel::OutOfBounds;
        }
        if point[0].round() as usize >= canvas.width || point[1].round() as usize >= canvas.height {
            return Pixel::OutOfBounds;
        }

        Pixel::Coordinate {
            x: point[0].round() as usize,
            y: canvas.height - 1 - point[1].round() as usize,
        }
    }
}

fn main() {
    let mut canvas = Canvas::new(400, 400);
    let mut pt = Point::new(100.0, 0.0, 0.0);
    let origin = Point::new(200.0, 200.0, 0.0);
    let color = Color::new(1.0, 0.0, 0.0);

    for i in 0..12 {
        let rot = Matrix::rotate(raytracer::matrix::Rotation::Z, (2.0 * PI) / 12.0);
        pt = rot * pt;
        let px = Pixel::from_point_for_canvas(pt, &origin, &canvas);
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
        let v: Vector<_> = pt.into();
        let len;
        match (i + 1) % 3 {
            0 => len = 10,
            _ => len = 5,
        }
        for i in 1..=len {
            let scaled = v.normalize() * (v.magnitude() - i as f64);
            let l_px = Pixel::from_point_for_canvas(scaled.into(), &origin, &canvas);
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
