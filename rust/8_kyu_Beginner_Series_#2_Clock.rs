fn past(h: i32, m: i32, s: i32) -> i32 {
    let mut sum = 0;
    if h <= 0 || h>=23{
        sum+=0
    }else{sum+=h*3_600_000;} 
    if m <= 0 || m>=59{
         sum+=0 
    }else{sum+=m*6_0000;}
    if s <= 0 || s>=59{
         sum+=0
    }else{sum+=s*1_000;}
    sum
//     (h * 3_600_000) + (m * 60_000) + (s * 1_000)
}