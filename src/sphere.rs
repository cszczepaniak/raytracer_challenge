use crate::{
    intersection::{Intersectable, Intersection, Intersections},
    material::{Illuminated, Phong},
    matrix::Matrix,
    point::Point,
    ray::Ray,
    utils::FuzzyEq,
    vector::Vector,
};

#[derive(Clone, Copy, Debug)]
pub struct Sphere<T = Phong>
where
    T: Illuminated,
{
    transform: Matrix<4>,
    material: T,
}

impl<T> Default for Sphere<T>
where
    T: Illuminated + Default,
{
    fn default() -> Self {
        Self {
            transform: Matrix::identity(),
            material: T::default(),
        }
    }
}

impl<T> FuzzyEq for Sphere<T>
where
    T: Illuminated + Copy,
{
    fn fuzzy_eq(&self, other: Self) -> bool {
        self.transform.fuzzy_eq(other.transform)
    }
}

impl<T> Intersectable<Sphere<T>> for Sphere<T>
where
    T: Illuminated + Copy,
{
    fn intersect(&self, r: Ray) -> Intersections<Sphere<T>> {
        let object_space_ray = r.transform(self.transform.inverse());

        let sphere_to_ray = object_space_ray.origin - Point::new(0.0, 0.0, 0.0);
        let a = object_space_ray.direction.dot(&object_space_ray.direction);
        let b = 2.0 * object_space_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let descriminant = b * b - 4.0 * a * c;
        if descriminant < 0.0 {
            vec![].into()
        } else {
            let t1 = (-b - descriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + descriminant.sqrt()) / (2.0 * a);
            vec![Intersection::new(t1, self), Intersection::new(t2, self)].into()
        }
    }

    fn normal_at(&self, p: Point) -> Vector {
        let t_inv = self.transform.inverse();
        let object_point = t_inv * p;
        let object_normal = (object_point - Point::new(0.0, 0.0, 0.0)).normalize();
        let world_normal = t_inv.transpose() * object_normal;
        Vector::new(world_normal[0], world_normal[1], world_normal[2]).normalize()
    }
}

impl<T> Sphere<T>
where
    T: Illuminated + Default,
{
    pub fn with_transform(self, transform: Matrix<4>) -> Self {
        Self {
            transform,
            material: self.material,
        }
    }

    pub fn with_material(self, material: T) -> Self {
        Self {
            material,
            transform: self.transform,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_1_SQRT_2, PI};

    use super::*;
    use crate::{
        assert_fuzzy_eq,
        color::Color,
        material::{Phong, PhongAttribute},
        matrix::Rotation,
        ray::Ray,
        utils::FuzzyEq,
        vector::Vector,
    };

    const FRAC_1_SQRT_3: f64 = 0.57735026919;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s: Sphere = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_fuzzy_eq!(4.0, xs[0].t);
        assert_fuzzy_eq!(6.0, xs[1].t);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s: Sphere = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_fuzzy_eq!(5.0, xs[0].t);
        assert_fuzzy_eq!(5.0, xs[1].t);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s: Sphere = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s: Sphere = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_fuzzy_eq!(-1.0, xs[0].t);
        assert_fuzzy_eq!(1.0, xs[1].t);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s: Sphere = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_fuzzy_eq!(-6.0, xs[0].t);
        assert_fuzzy_eq!(-4.0, xs[1].t);
    }

    #[test]
    fn a_spheres_default_transform() {
        let s: Sphere = Sphere::default();
        assert_fuzzy_eq!(s.transform, Matrix::<4>::identity());
    }

    #[test]
    fn changing_a_spheres_transform() {
        let mut s: Sphere<Phong> = Sphere::default();
        let m = Matrix::translate(2.0, 3.0, 4.0);
        s.transform = m;

        assert_fuzzy_eq!(s.transform, m);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s: Sphere = Sphere::default().with_transform(Matrix::scale(2.0, 2.0, 2.0));

        let xs = s.intersect(r);

        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s: Sphere = Sphere::default().with_transform(Matrix::translate(5.0, 0.0, 0.0));

        let xs = s.intersect(r);

        assert_eq!(0, xs.len());
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s: Sphere = Sphere::default();
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));

        let expected_result = Vector::new(1.0, 0.0, 0.0);

        assert_fuzzy_eq!(expected_result, n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s: Sphere = Sphere::default();
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));

        let expected_result = Vector::new(0.0, 1.0, 0.0);

        assert_fuzzy_eq!(expected_result, n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s: Sphere = Sphere::default();
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));

        let expected_result = Vector::new(0.0, 0.0, 1.0);

        assert_fuzzy_eq!(expected_result, n);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_non_axial_point() {
        let s: Sphere = Sphere::default();
        let p = Point::new(FRAC_1_SQRT_3, FRAC_1_SQRT_3, FRAC_1_SQRT_3);
        let n = s.normal_at(p);

        let expected_result = Vector::new(FRAC_1_SQRT_3, FRAC_1_SQRT_3, FRAC_1_SQRT_3);

        assert_fuzzy_eq!(expected_result, n);
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s: Sphere = Sphere::default().with_transform(Matrix::translate(0.0, 1.0, 0.0));
        let p = Point::new(0.0, 1.70711, -0.70711);
        let n = s.normal_at(p);

        let expected_result = Vector::new(0.0, 0.70711, -0.70711);

        assert_fuzzy_eq!(expected_result, n);
    }

    #[test]
    fn computing_the_normal_on_a_scaled_and_rotated_sphere() {
        let s: Sphere = Sphere::default()
            .with_transform(Matrix::scale(1.0, 0.5, 1.0) * Matrix::rotate(Rotation::Z, PI / 5.0));
        let p = Point::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let n = s.normal_at(p);

        let expected_result = Vector::new(0.0, 0.97014, -0.24254);

        assert_fuzzy_eq!(expected_result, n);
    }

    #[test]
    fn the_normal_vector_is_always_normalized() {
        let s: Sphere = Sphere::default();
        let p = Point::new(FRAC_1_SQRT_3, FRAC_1_SQRT_3, FRAC_1_SQRT_3);
        let n = s.normal_at(p);

        assert_fuzzy_eq!(n.normalize(), n);
    }

    #[test]
    fn the_normal_vector_is_normalized_on_transformed_sphere() {
        let s: Sphere = Sphere::default()
            .with_transform(Matrix::scale(1.0, 0.5, 1.0) * Matrix::rotate(Rotation::Z, PI / 5.0));
        let p = Point::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let n = s.normal_at(p);

        assert_fuzzy_eq!(n.normalize(), n);
    }

    #[test]
    fn sphere_has_default_phong_material() {
        let s: Sphere = Sphere::default();
        let m = Phong::default();

        assert_fuzzy_eq!(m, s.material);
    }

    #[test]
    fn sphere_may_be_assigned_a_material() {
        let phong = Phong::new(&[
            PhongAttribute::Color(Color::new(1.0, 1.0, 0.0)),
            PhongAttribute::Ambient(0.05),
            PhongAttribute::Diffuse(0.7),
            PhongAttribute::Specular(0.95),
            PhongAttribute::Shininess(400.0),
        ]);
        let s = Sphere::default().with_material(phong);

        assert_fuzzy_eq!(phong, s.material);
    }
}
