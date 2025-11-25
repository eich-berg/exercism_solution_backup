pub fn check(candidate: &str) -> bool {
    // todo!("Is {candidate} an isogram?");
    let char_list: Vec<char> = candidate.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();
    //let mut char_list_dedup: Vec<char> = char_list.iter().cloned().collect();
    let mut char_list_dedup: Vec<char> = char_list.to_vec();
    char_list_dedup.sort();
    char_list_dedup.dedup();
    char_list.len() == char_list_dedup.len()
    
}