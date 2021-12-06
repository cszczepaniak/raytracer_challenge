use crate::{
    fuzzy_eq::FuzzyEq,
    intersection::{Intersectable, Intersections, Normal},
    material::Material,
    ray::Ray,
    sphere::Sphere,
};

#[derive(Clone, Copy, Debug)]
pub enum Body {
    Sphere(Sphere),
}

impl Body {
    pub fn material(&self) -> Material {
        match self {
            Body::Sphere(s) => s.material,
        }
    }
}

impl From<Sphere> for Body {
    fn from(s: Sphere) -> Self {
        Body::Sphere(s)
    }
}

impl FuzzyEq for Body {
    fn fuzzy_eq(&self, other: Self) -> bool {
        match self {
            Body::Sphere(s) => match other {
                Body::Sphere(os) => s.fuzzy_eq(os),
            },
        }
    }
}

impl Intersectable for Body {
    fn intersect(&self, r: Ray) -> Intersections {
        match self {
            Body::Sphere(s) => s.intersect(r),
        }
    }
}

impl Normal for Body {
    fn normal_at(&self, p: crate::point::Point) -> crate::vector::Vector {
        match self {
            Body::Sphere(s) => s.normal_at(p),
        }
    }
}
