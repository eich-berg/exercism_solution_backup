/// Determine whether a sentence is a pangram.
pub fn is_pangram(mut sentence: &str) -> bool {
    // todo!("Is {sentence} a pangram?");
    let sentence = sentence.to_lowercase();
    ('a'..='z').all(|c| sentence.contains(c))
}


