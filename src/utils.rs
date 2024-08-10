pub fn round_to(num: f64, decimals: i32) -> f64 {
    let factor = 10f64.powi(decimals);
    (num * factor).round() / factor
}
