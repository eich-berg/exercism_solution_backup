pub fn egg_count(display_value: u32) -> usize {
    //todo!("count the eggs in {display_value}")
    let binary_string = format!("{display_value:b}");
    binary_string.matches('1').count()
}