use std::ops::{Index, IndexMut};

use crate::{
    body::Body,
    computed_intersection::{ComputedIntersection, Orientation},
    fuzzy_eq::{FuzzyEq, EPISILON},
    point::Point,
    ray::Ray,
    vector::Vector,
};

pub trait Intersectable {
    fn intersect(&self, r: Ray) -> Intersections;
}

pub trait Normal {
    fn normal_at(&self, p: Point) -> Vector;
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub t: f64,
    pub ray: Ray,
    pub body: Body,
}

impl Intersection {
    pub fn new(t: f64, ray: Ray, body: Body) -> Self {
        Self { t, ray, body }
    }

    pub fn computed(&self) -> ComputedIntersection {
        let position = self.ray.position(self.t);
        let mut normal = self.body.normal_at(position);
        let eye = -self.ray.direction;

        let orientation = if normal.dot(&eye) < 0.0 {
            normal = -normal;
            Orientation::Inside
        } else {
            Orientation::Outside
        };

        let over_point = position + normal * EPISILON;

        ComputedIntersection::new(self, position, over_point, normal, eye, orientation)
    }
}

impl FuzzyEq for &Intersection {
    fn fuzzy_eq(&self, other: Self) -> bool {
        self.t.fuzzy_eq(other.t) && self.body.fuzzy_eq(other.body)
    }
}

pub struct Intersections {
    intersections: Vec<Intersection>,
}

impl Intersections {
    pub fn hit(&self) -> Option<&Intersection> {
        for intersection in self.intersections.iter() {
            if intersection.t > 0.0 {
                return Some(intersection);
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.intersections.is_empty()
    }
}

impl From<Vec<Intersection>> for Intersections {
    fn from(mut intersections: Vec<Intersection>) -> Self {
        intersections.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Self { intersections }
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

impl IndexMut<usize> for Intersections {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.intersections[index]
    }
}

impl IntoIterator for Intersections {
    type Item = Intersection;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.intersections.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_fuzzy_eq, fuzzy_eq::FuzzyEq, sphere::Sphere};

    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_body() {
        let b = Body::from(Sphere::default());
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let i = Intersection::new(3.5, r, b);
        assert_fuzzy_eq!(3.5, i.t);
        assert_fuzzy_eq!(b, i.body);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let b = Body::from(Sphere::default());
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let i1 = Intersection::new(1.0, r, b);
        let i2 = Intersection::new(2.0, r, b);

        let xs: Intersections = vec![i2, i1].into();

        assert_fuzzy_eq!(Some(&i1), xs.hit());
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let b = Body::from(Sphere::default());
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let i1 = Intersection::new(-1.0, r, b);
        let i2 = Intersection::new(1.0, r, b);

        let xs: Intersections = vec![i2, i1].into();

        assert_fuzzy_eq!(Some(&i2), xs.hit());
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let b = Body::from(Sphere::default());
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));

        let i1 = Intersection::new(-2.0, r, b);
        let i2 = Intersection::new(-1.0, r, b);

        let xs: Intersections = vec![i2, i1].into();

        let exp: Option<&Intersection> = None;
        assert_fuzzy_eq!(xs.hit(), exp);
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let body = Body::from(Sphere::default());
        let i = Intersection::new(4.0, r, body);
        let c = i.computed();

        assert_fuzzy_eq!(&i, c.intersection);
        assert_fuzzy_eq!(Point::new(0.0, 0.0, -1.0), c.position);
        assert_fuzzy_eq!(Vector::new(0.0, 0.0, -1.0), c.eye);
        assert_fuzzy_eq!(Vector::new(0.0, 0.0, -1.0), c.normal);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let body = Body::from(Sphere::default());
        let i = Intersection::new(4.0, r, body);
        let c = i.computed();

        assert_eq!(Orientation::Outside, c.orientation);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let body = Body::from(Sphere::default());
        let i = Intersection::new(1.0, r, body);
        let c = i.computed();

        assert_eq!(Orientation::Inside, c.orientation);
        assert_fuzzy_eq!(Vector::new(0.0, 0.0, -1.0), c.normal);
    }

    //   #[test]
    //   fn the_hit_should_offset_the_point() {
    //     let material = Material::default();
    //     let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    //     let s1 = Sphere::new(material, Matrix::translation(0.0, 0.0, 1.0));
    //     let i = Intersection::new(5.0, r, s1.into());
    //     let c = i.get_computed();
    //     assert!(c.over_point.z < -EPSILON / 2.0);
    //     assert!(c.point.z > c.over_point.z);
    //   }
}
