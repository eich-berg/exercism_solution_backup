pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    
    let len = digits.len() as u32;
    
    let sum: u32 = digits
        .iter()
        .map(|d| d.pow(len))
        .sum();
    
    num == sum
}