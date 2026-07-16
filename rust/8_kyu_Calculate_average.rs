fn find_average(slice: &[f64]) -> f64 {
    if slice.is_empty() {
        return 0.0;
    }
    let mut result = 0.0;
    for &i in slice{
        result+=i;
    }
    result/slice.len() as f64
//     slice.iter().sum::<f64>() / (slice.len() as f64)
}