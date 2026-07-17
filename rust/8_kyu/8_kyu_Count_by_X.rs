fn count_by(x: u32, n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    for i in 1..=n {
        result.push(x*i);
    }
    result
}