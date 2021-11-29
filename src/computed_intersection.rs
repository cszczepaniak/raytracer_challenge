use crate::{intersection::Intersection, point::Point, vector::Vector};

#[derive(Clone, Copy, Debug)]
pub struct ComputedIntersection<'a> {
    pub intersection: &'a Intersection,
    pub position: Point,
    pub normal: Vector,
    pub eye: Vector,
}

impl<'a> ComputedIntersection<'a> {
    pub fn new(
        intersection: &'a Intersection,
        position: Point,
        normal: Vector,
        eye: Vector,
    ) -> Self {
        Self {
            intersection,
            position,
            normal,
            eye,
        }
    }
}
