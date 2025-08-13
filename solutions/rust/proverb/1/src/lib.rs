pub fn build_proverb(list: &[&str]) -> String {
    // todo!("build a proverb from this list of items: {list:?}")
    let mut output = String::new();
    if list.len() == 0 { return output; }
    for window in list.windows(2) {
        let verse = format!("For want of a {} the {} was lost.\n", window[0], window[1]);
        output.push_str(&verse);
    }
    let epi = format!("And all for the want of a {}.", list[0]);
    output.push_str(&epi);
    output.trim_end().to_string()
}

