pub fn brackets_are_balanced(string: &str) -> bool {
    // todo!("Check if the string \"{string}\" contains balanced brackets");
    let mut bracket_stk = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => bracket_stk.push(c),
            ')' => if bracket_stk.pop() != Some('(') { return false },
            ']' => if bracket_stk.pop() != Some('[') { return false },
            '}' => if bracket_stk.pop() != Some('{') { return false },
            _ => ()
        }
    }
   bracket_stk.is_empty()
}