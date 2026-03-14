pub fn is_pangram(sentence: &str) -> bool {
    // todo!("Is {sentence} a pangram?");
    ('a'..='z').all(|c| sentence.to_lowercase().contains(c))
}