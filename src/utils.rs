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

#[macro_export]
macro_rules! assert_fuzzy_eq {
    ($x:expr, $y:expr) => {
        assert!(($x).fuzzy_eq(($y)), "want: {:?}, got: {:?}", $x, $y);
    };
}
