use crate::{
    body::Body,
    intersection::{Intersectable, Intersection, Intersections},
    light::PointLight,
    ray::Ray,
};

#[derive(Default)]
pub struct World {
    pub bodies: Vec<Body>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new(bodies: Vec<Body>, lights: Vec<PointLight>) -> Self {
        Self { bodies, lights }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let xss: Vec<Intersection> = self
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
        material::{Material, Phong, PhongAttribute},
        matrix::Matrix,
        point::Point,
        ray::Ray,
        sphere::Sphere,
        utils::FuzzyEq,
        vector::Vector,
    };

    use super::*;

    fn create_default_world() -> World {
        let light = PointLight::new(Point::new(1.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0));
        let material = Material::Phong(Phong::new(&[
            PhongAttribute::Color(Color::new(0.8, 1.0, 0.6)),
            PhongAttribute::Diffuse(0.7),
            PhongAttribute::Specular(0.2),
        ]));
        let s1: Body = Sphere::default().with_material(material).into();
        let s2: Body = Sphere::default()
            .with_transform(Matrix::scale(0.5, 0.5, 0.5))
            .into();

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
