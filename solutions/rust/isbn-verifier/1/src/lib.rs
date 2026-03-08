/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // todo!("Is {isbn:?} a valid ISBN number?");
    let isbn_copy = isbn.replace("-", "");
    if isbn_copy.len() != 10 { return false; }
    let mut sum = 0;
    for (i, val) in isbn_copy.chars().enumerate() {
        let digit = match (val, i) {
            ('X', 9) => 10,
            ('0'..='9', _) => val.to_digit(10).unwrap(),
            _ => return false,
        };
        sum += digit * (10 - i) as u32;
    }
    sum % 11 == 0
}