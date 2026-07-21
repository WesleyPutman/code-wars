fn descending_order(x: u64) -> u64 {
    let mut table = x.to_string().chars().collect::<Vec<char>>();
    table.sort();
    table.reverse();
    table.into_iter().collect::<String>().parse::<u64>().unwrap()
}