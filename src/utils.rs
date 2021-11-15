use crate::float::Float;

pub trait FuzzyEq: Copy {
    fn fuzzy_eq(&self, other: Self) -> bool;
    fn fuzzy_ne(&self, other: Self) -> bool {
        !self.fuzzy_eq(other)
    }
}

impl FuzzyEq for f64 {
    fn fuzzy_eq(&self, other: Self) -> bool {
        (self - other).abs() < 0.00001
    }
}

impl<T, const N: usize> FuzzyEq for [T; N]
where
    T: Float + FuzzyEq,
{
    fn fuzzy_eq(&self, other: Self) -> bool {
        for i in 0..N {
            if self[i].fuzzy_ne(other[i]) {
                return false;
            }
        }
        true
    }
}

#[macro_export]
macro_rules! assert_fuzzy_eq {
    ($x:expr, $y:expr) => {
        assert!(($x).fuzzy_eq(($y)), "want: {:?}, got: {:?}", $x, $y);
    };
}
