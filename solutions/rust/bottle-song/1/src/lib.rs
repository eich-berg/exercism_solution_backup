pub fn recite(start_bottles: u32, take_down: u32) -> String {
    // todo!("Return the bottle song starting at {start_bottles} and taking down {take_down} bottles")
    let bottles = vec!["Ten", "Nine", "Eight", "Seven", "Six", "Five", "Four", "Three", "Two", "One", "No"];
    let mut result = String::new();
    for i in 0..take_down {
        let current_index = (10 - (start_bottles - i)) as usize;
        let next_index = (10 - (start_bottles - i - 1)) as usize;
        
        let current_bottle = if bottles[current_index] == "One" { "bottle" } else { "bottles" };
        let next_bottle = if bottles[next_index] == "One" { "bottle" } else { "bottles" };
        
        result.push_str(&format!(
            "{} green {} hanging on the wall,\n{} green {} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {} green {} hanging on the wall.\n",
            bottles[current_index], current_bottle,
            bottles[current_index], current_bottle,
            bottles[next_index].to_lowercase(), next_bottle
        ));
        if i < take_down - 1 { result.push('\n'); }
    }
    result
}