pub fn f64_fuzzy_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.00001
}

pub trait FuzzyEq {
    fn fuzzy_eq(&self, other: &Self) -> bool;
}

#[macro_export]
macro_rules! assert_fuzzy_eq {
    ($x:expr, $y:expr) => {
        assert!(($x).fuzzy_eq(($y)))
    };
}
