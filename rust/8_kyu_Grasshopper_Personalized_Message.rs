fn greet(name: &str, owner: &str) -> String {
    if name != owner{
        return String::from("Hello guest");
    }else{
        return String::from("Hello boss");
    }
}