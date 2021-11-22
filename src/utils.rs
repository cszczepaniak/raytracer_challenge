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
    T: FuzzyEq,
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

impl<T> FuzzyEq for Option<T>
where
    T: Copy + FuzzyEq,
{
    fn fuzzy_eq(&self, other: Self) -> bool {
        match (self, other) {
            (None, None) => true,
            (Some(a), Some(b)) => a.fuzzy_eq(b),
            (_, _) => false,
        }
    }
}

#[macro_export]
macro_rules! assert_fuzzy_eq {
    ($x:expr, $y:expr) => {
        assert!(($x).fuzzy_eq(($y)), "want: {:?}, got: {:?}", $x, $y);
    };
}
