use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

// Define a trait that will allow our implementations of vectors, matrices, etc.
// to be more generic.
// This trait inherits from some subset of the ops that are possible for both
// f32 and f64 so we can use +, -, *, /, etc. in our generic implementations.
// We also add an identity function, which we need for identity matrices.
pub trait Float:
    Default
    + Copy
    + Add<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Sub<Output = Self>
    + Neg<Output = Self>
    + AddAssign<Self>
{
    fn identity() -> Self;
    fn sqrt(self) -> Self;
}
impl Float for f32 {
    fn identity() -> Self {
        1.0
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl Float for f64 {
    fn identity() -> Self {
        1.0
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
