use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut word_sorted: Vec<char> = word.to_lowercase().chars().collect();
    word_sorted.sort_unstable();
    possible_anagrams.iter().filter(|&&c| {
        c.len() == word.len() &&
        c.to_lowercase() != word.to_lowercase() &&
        {
            let mut c_sorted: Vec<char> = c.to_lowercase().chars().collect();
            c_sorted.sort_unstable();
            c_sorted == word_sorted
 
        }
    }).copied().collect()
}