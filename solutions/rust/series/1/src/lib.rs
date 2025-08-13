pub fn series(digits: &str, len: usize) -> Vec<String> {
    // todo!("What are the series of length {len} in string {digits:?}")
    let mut output: Vec<String> = Vec::new();
    if digits.is_empty() || len > digits.len() { return output; }
    if len == digits.len(){ 
        output.push(digits.to_string()); 
        return output;
    }
    // Convert to Vec<char> to use windows()
    let chars: Vec<char> = digits.chars().collect();
    for window in chars.windows(len) {
        output.push(window.iter().collect());
    }
    output
}

    
