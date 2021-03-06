use crate::{intersection::Intersection, point::Point, vector::Vector};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Inside,
    Outside,
}

#[derive(Clone, Copy, Debug)]
pub struct ComputedIntersection<'a> {
    pub intersection: &'a Intersection,
    pub position: Point,
    pub over_point: Point,
    pub normal: Vector,
    pub eye: Vector,
    pub orientation: Orientation,
}

impl<'a> ComputedIntersection<'a> {
    pub fn new(
        intersection: &'a Intersection,
        position: Point,
        over_point: Point,
        normal: Vector,
        eye: Vector,
        orientation: Orientation,
    ) -> Self {
        Self {
            intersection,
            position,
            over_point,
            normal,
            eye,
            orientation,
        }
    }
}
