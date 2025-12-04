/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // todo!("Is {isbn:?} a valid ISBN number?");
    let isbn_copy = isbn.replace("-", "");
    if isbn_copy.len() != 10 { return false; }
    let mut sum = 0;
    for (i, val) in isbn_copy.chars().enumerate() {
        if i == 9 && val == 'X' { sum += 10; }
        else if val.is_numeric() { sum += val.to_digit(10).unwrap() * (10 - i) as u32; }
        else { return false; }
    }
    sum % 11 == 0
}