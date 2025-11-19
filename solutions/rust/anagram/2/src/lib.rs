use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
    let mut w: Vec<char> = word.to_lowercase().chars().collect();
    w.sort();
    possible_anagrams.iter().filter(|&&el| {
        el.len() == word.len() && el.to_lowercase() != word.to_lowercase() && {
            let mut e: Vec<char> = el.to_lowercase().chars().collect();
            e.sort();
            e == w
        }
    }).cloned().collect()
}