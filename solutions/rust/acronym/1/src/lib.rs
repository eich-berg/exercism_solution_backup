pub fn abbreviate(phrase: &str) -> String {
    // todo!("Given the phrase '{phrase}', return its acronym");
    let mut filtered = String::new();
    let mut prev_char: Option<char> = None;
    for c in phrase.chars() {
        match c {
            '_' | '-' => filtered.push(' '),
            c => {
                if let Some(prev) = prev_char {
                    if prev.is_lowercase() && c.is_uppercase() {
                        filtered.push(' ');
                    }
                }
                filtered.push(c);
                prev_char = Some(c);
            }
        }
    }
    filtered.split_whitespace().filter_map(|word| word.chars().next()).map(|c| c.to_uppercase().to_string()).collect()
    
    }