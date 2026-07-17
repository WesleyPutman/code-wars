fn better_than_average(class_points: &[u16], your_points: u16) -> bool {
    your_points as usize > class_points.iter().sum::<u16>() as usize / class_points.len()
}