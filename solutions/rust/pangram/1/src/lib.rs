/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    // todo!("Is {sentence} a pangram?");
    let mut char_list: Vec<char> = sentence.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();
    char_list.sort();
    char_list.dedup();
    char_list.len() == 26
}