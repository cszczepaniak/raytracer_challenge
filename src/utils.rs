pub fn f64_fuzzy_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.00001
}
