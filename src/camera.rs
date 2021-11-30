use crate::{matrix::Matrix, point::Point, ray::Ray};

pub struct Camera {
    pub transform: Matrix<4>,
    pub vsize: usize,
    pub hsize: usize,
    pub field_of_view: f64,

    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_size = (field_of_view / 2.0).tan();
        let aspect_ratio = hsize as f64 / vsize as f64;

        let (half_width, half_height) = if aspect_ratio >= 1.0 {
            (half_size, half_size / aspect_ratio)
        } else {
            (half_size * aspect_ratio, half_size)
        };

        let pixel_size = half_width * 2.0 / hsize as f64;

        Self {
            transform: Matrix::identity(),
            vsize,
            hsize,
            field_of_view,
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn with_transform(self, transform: Matrix<4>) -> Self {
        Self { transform, ..self }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let offset_x = (0.5 + x as f64) * self.pixel_size;
        let offset_y = (0.5 + y as f64) * self.pixel_size;
        let world_x = self.half_width - offset_x;
        let world_y = self.half_height - offset_y;

        let inverse_view_transform = self.transform.inverse();

        let wall_point = inverse_view_transform * Point::new(world_x, world_y, -1.0);
        let ray_origin = inverse_view_transform * Point::new(0.0, 0.0, 0.0);
        Ray::new(ray_origin, (wall_point - ray_origin).normalize())
    }
}

#[cfg(test)]
mod tests {
    use crate::{matrix::Rotation, utils::FuzzyEq, vector::Vector};
    use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4};

    use crate::assert_fuzzy_eq;

    use super::*;

    #[test]
    fn constructing_a_camera() {
        let vsize = 200;
        let hsize = 400;
        let fov = FRAC_PI_2;

        let camera = Camera::new(hsize, vsize, fov);
        assert_eq!(vsize, camera.vsize);
        assert_eq!(hsize, camera.hsize);
        assert_fuzzy_eq!(fov, camera.field_of_view);
        assert_fuzzy_eq!(Matrix::<4>::identity(), camera.transform);
    }

    #[test]
    fn constructing_a_camera_with_transform() {
        let transform = Matrix::translate(1.0, 2.0, 5.0);

        let camera = Camera::new(200, 400, FRAC_PI_2).with_transform(transform);
        assert_fuzzy_eq!(transform, camera.transform);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, FRAC_PI_2);
        let r = c.ray_for_pixel(100, 50);

        assert_fuzzy_eq!(Point::new(0.0, 0.0, 0.0), r.origin);
        assert_fuzzy_eq!(Vector::new(0.0, 0.0, -1.0), r.direction);
    }

    #[test]
    fn constructing_a_ray_through_the_corner_of_a_canvas() {
        let c = Camera::new(201, 101, FRAC_PI_2);
        let r = c.ray_for_pixel(0, 0);

        assert_fuzzy_eq!(Point::new(0.0, 0.0, 0.0), r.origin);
        assert_fuzzy_eq!(Vector::new(0.66519, 0.33259, -0.66851), r.direction);
    }

    #[test]
    fn constructing_a_ray_when_camera_is_transformed() {
        let c = Camera::new(201, 101, FRAC_PI_2).with_transform(
            Matrix::rotate(Rotation::Y, FRAC_PI_4) * Matrix::translate(0.0, -2.0, 5.0),
        );
        let r = c.ray_for_pixel(100, 50);

        assert_fuzzy_eq!(Point::new(0.0, 2.0, -5.0), r.origin);
        assert_fuzzy_eq!(Vector::new(FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2), r.direction);
    }
}
