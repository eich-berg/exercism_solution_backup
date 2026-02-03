pub fn egg_count(display_value: u32) -> usize {
    // todo!("count the eggs in {display_value}")
    let binary = format!("{:b}", display_value);
    binary.chars().filter(|b| *b == '1').count()
}
