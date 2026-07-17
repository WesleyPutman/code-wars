fn count_sheep(sheep: &[bool]) -> u8 {
    let mut total: u8 = 0;
    for &item in sheep.iter(){
        if item == true{
            total +=1;
        }
    }
    total
}