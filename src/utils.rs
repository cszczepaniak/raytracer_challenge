pub trait FuzzyEq: Copy {
    fn fuzzy_eq(&self, other: Self) -> bool;
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
