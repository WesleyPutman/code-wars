fn are_you_playing_banjo(name: &str) -> String {
    if name.starts_with(['R','r']){
        format!("{} plays banjo", name)
    }else{
        format!("{} does not play banjo", name)
    }
}

fn are_you_playing_banjo(name: &str) -> String {
    match name.starts_with(['R', 'r']) {
        true => format!("{} plays banjo", name),
        false => format!("{} does not play banjo", name),
    }
}