use crate::{
    intersection::{Intersectable, Intersection, Intersections, Normal},
    light::PointLight,
    ray::Ray,
    utils::FuzzyEq,
};

#[derive(Default)]
pub struct World<T>
where
    T: Intersectable<T> + Normal + FuzzyEq,
{
    pub bodies: Vec<T>,
    pub lights: Vec<PointLight>,
}

impl<'a, T> World<T>
where
    T: Intersectable<T> + Normal + FuzzyEq,
{
    pub fn new(bodies: Vec<T>, lights: Vec<PointLight>) -> Self {
        Self { bodies, lights }
    }

    pub fn intersect(&'a self, ray: Ray) -> Intersections<'a, T> {
        let xss: Vec<Intersection<'a, T>> = self
            .bodies
            .iter()
            .flat_map(|body| body.intersect(ray))
            .collect();
        Intersections::from(xss)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_fuzzy_eq,
        color::Color,
        material::{Phong, PhongAttribute},
        matrix::Matrix,
        point::Point,
        ray::Ray,
        sphere::Sphere,
        vector::Vector,
    };

    use super::*;

    fn create_default_world() -> World<Sphere> {
        let light = PointLight::new(Point::new(1.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0));
        let material = Phong::new(&[
            PhongAttribute::Color(Color::new(0.8, 1.0, 0.6)),
            PhongAttribute::Diffuse(0.7),
            PhongAttribute::Specular(0.2),
        ]);
        let s1 = Sphere::default().with_material(material);
        let s2: Sphere = Sphere::default().with_transform(Matrix::scale(0.5, 0.5, 0.5));

        World::new(vec![s1, s2], vec![light])
    }

    #[test]
    fn make_a_world() {
        let world = create_default_world();

        assert_eq!(2, world.bodies.len());
        assert_eq!(1, world.lights.len());
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let world = create_default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let xs = world.intersect(r);
        assert_eq!(4, xs.len());
        assert_fuzzy_eq!(4.0, xs[0].t);
        assert_fuzzy_eq!(4.5, xs[1].t);
        assert_fuzzy_eq!(5.5, xs[2].t);
        assert_fuzzy_eq!(6.0, xs[3].t);
    }
}
