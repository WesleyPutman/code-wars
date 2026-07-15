fn are_you_playing_banjo(name: &str) -> String {
    if name.starts_with(['R','r']){
        format!("{} plays banjo", name)
    }else{
        format!("{} does not play banjo", name)
    }
}