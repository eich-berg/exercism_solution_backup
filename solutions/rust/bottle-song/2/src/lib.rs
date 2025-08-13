use std::collections::HashMap;

pub fn recite(mut start_bottles: u32, take_down: u32) -> String {
    // todo!("Return the bottle song starting at {start_bottles} and taking down {take_down} bottles")
    let mut output = String::new();

    let bottles: HashMap<u32, &str> = [
        (10, "Ten"), (9, "Nine"), (8, "Eight"), (7, "Seven"),
        (6, "Six"), (5, "Five"), (4, "Four"), (3, "Three"),
        (2, "Two"), (1, "One"), (0, "no")
    ].iter().cloned().collect();
    
    for _ in 0..take_down {
        let bottle_p1 = if start_bottles == 1 { "bottle" } else { "bottles" };
        let verse1 = format!(
            "{} green {} hanging on the wall,\n\
             {} green {} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n",
            bottles[&start_bottles], bottle_p1, bottles[&start_bottles], bottle_p1);
        let bottle_p2 = if start_bottles-1 == 1 { "bottle" } else { "bottles" };
        let verse2 = format!("There'll be {} green {} hanging on the wall.\n\n",
            bottles[&(start_bottles-1)].to_lowercase(), bottle_p2);
        output.push_str(&verse1);
        output.push_str(&verse2);
        start_bottles -= 1; 
        if start_bottles == 0 { break; }
    }
    output.trim_end().to_string() // Remove final extra newline
}