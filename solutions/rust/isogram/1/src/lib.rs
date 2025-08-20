use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    // todo!("Is {candidate} an isogram?");
    if !candidate.is_empty() {
        let filtered_candidate: Vec<char> = candidate.to_lowercase()
            .chars()
            .filter(|c| !c.is_whitespace() && *c != '-')
            .collect();
        let unique_chars: HashSet<char> = filtered_candidate.iter().cloned().collect();
        return filtered_candidate.len() == unique_chars.len();
    }
    true
}