use std::ops::{Index, IndexMut};

use crate::{point::Point, ray::Ray, utils::FuzzyEq, vector::Vector};

pub trait Intersectable<T>
where
    T: Intersectable<T> + FuzzyEq,
{
    fn intersect(&self, r: Ray) -> Intersections<T>;
    fn normal_at(&self, p: Point) -> Vector;
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection<'a, T>
where
    T: Intersectable<T> + FuzzyEq,
{
    pub t: f64,
    body: &'a T,
}

impl<'a, T> Intersection<'a, T>
where
    T: Intersectable<T> + FuzzyEq,
{
    pub fn new(t: f64, body: &'a T) -> Self {
        Self { t, body }
    }
}

impl<'a, T> FuzzyEq for &Intersection<'a, T>
where
    T: FuzzyEq + Intersectable<T>,
{
    fn fuzzy_eq(&self, other: Self) -> bool {
        self.t.fuzzy_eq(other.t) && self.body.fuzzy_eq(*other.body)
    }
}

pub struct Intersections<'a, T>
where
    T: Intersectable<T> + FuzzyEq,
{
    intersections: Vec<Intersection<'a, T>>,
}

impl<'a, T> Intersections<'a, T>
where
    T: Intersectable<T> + FuzzyEq,
{
    pub fn hit(&self) -> Option<&Intersection<'a, T>> {
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

impl<'a, T> From<Vec<Intersection<'a, T>>> for Intersections<'a, T>
where
    T: Intersectable<T> + FuzzyEq,
{
    fn from(mut intersections: Vec<Intersection<'a, T>>) -> Self {
        intersections.sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Self { intersections }
    }
}

impl<'a, T> Index<usize> for Intersections<'a, T>
where
    T: Intersectable<T> + FuzzyEq,
{
    type Output = Intersection<'a, T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

impl<'a, T> IndexMut<usize> for Intersections<'a, T>
where
    T: Intersectable<T> + FuzzyEq,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.intersections[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_fuzzy_eq, sphere::Sphere, utils::FuzzyEq};

    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_body() {
        let s: Sphere = Sphere::default();
        let i = Intersection::new(3.5, &s);
        assert_fuzzy_eq!(3.5, i.t);
        assert_fuzzy_eq!(s, *i.body);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s: Sphere = Sphere::default();

        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs: Intersections<Sphere> = vec![i2, i1].into();

        assert_fuzzy_eq!(Some(&i1), xs.hit());
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s: Sphere = Sphere::default();

        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);

        let xs: Intersections<Sphere> = vec![i2, i1].into();

        assert_fuzzy_eq!(Some(&i2), xs.hit());
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s: Sphere = Sphere::default();

        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);

        let xs: Intersections<Sphere> = vec![i2, i1].into();

        let exp: Option<&Intersection<Sphere>> = None;
        assert_fuzzy_eq!(xs.hit(), exp);
    }

    //   #[test]
    //   fn precomputing_the_state_of_an_intersection() {
    //     let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    //     let body = Body::from(Sphere::default());
    //     let i = Intersection::new(4.0, r, body);
    //     let c = i.get_computed();

    //     assert_eq!(c.intersection, &i);
    //     assert_fuzzy_eq!(c.point, Point::new(0.0, 0.0, -1.0));
    //     assert_fuzzy_eq!(c.eyev, Vector::new(0.0, 0.0, -1.0));
    //     assert_fuzzy_eq!(c.normalv, Vector::new(0.0, 0.0, -1.0));
    //   }

    //   #[test]
    //   fn the_hit_when_an_intersection_occurs_on_the_outside() {
    //     let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    //     let body = Body::from(Sphere::default());
    //     let i = Intersection::new(4.0, r, body);
    //     let c = i.get_computed();

    //     assert_eq!(c.inside, false);
    //   }

    //   #[test]
    //   fn the_hit_when_an_intersection_occurs_on_the_inside() {
    //     let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    //     let body = Body::from(Sphere::default());
    //     let i = Intersection::new(1.0, r, body);
    //     let c = i.get_computed();

    //     assert_eq!(c.inside, true);
    //     assert_eq!(c.normalv, Vector::new(0.0, 0.0, -1.0));
    //   }

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
