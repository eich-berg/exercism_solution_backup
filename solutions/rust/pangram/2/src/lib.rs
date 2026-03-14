use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    // todo!("Is {sentence} a pangram?");
    sentence.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect::<HashSet<_>>().len() == 26
}