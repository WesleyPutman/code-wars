fn printer_error(s: &str) -> String {
    let mut error_count = 0;
    for i in s.chars(){
        if i > 'm' {error_count +=1}
    }
    format!("{}/{}", error_count, s.len())
}