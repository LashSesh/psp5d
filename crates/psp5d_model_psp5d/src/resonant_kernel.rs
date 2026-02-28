pub fn diameter(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    max - min
}

pub fn delta_pi(prev: f64, next: f64) -> f64 {
    (next - prev).abs()
}
