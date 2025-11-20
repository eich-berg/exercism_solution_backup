/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // todo!("Is the Luhn checksum for {code} valid?");
    if code.len() <= 1 { return false; }
    if code.chars().any(|c| !c.is_ascii_digit() && c != ' ') { return false; }
    let digits: Vec<u32> = code.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() <= 1 { return false; }
    let sum: u32 = digits.into_iter().rev().enumerate().map(|(i, x)| if i % 2 == 1 {
        if x * 2 > 9 { (x * 2) - 9 } else { x * 2 } } else { x }).sum();
    sum % 10 == 0 
}