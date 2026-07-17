fn simple_multiplication(number: u8) -> u8 {
    match number % 2 {
        1 => number*9,
        _ => number*8,
    }
}