fn is_square(n: i64) -> bool {
    if n < 0 { false } else {
        let square = (n as f64).sqrt();
        (square as i64) * (square as i64) == n 
    }
}
