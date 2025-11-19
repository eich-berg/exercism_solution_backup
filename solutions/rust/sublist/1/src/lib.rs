#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    // todo!("Determine if the {first_list:?} is equal to, sublist of, superlist of or unequal to {second_list:?}.");
    match (first_list, second_list) {
        (a, b) if a == b => Comparison::Equal,
        (a, b) if a.len() == 0 || b.windows(a.len()).any(|window| window == a) => Comparison::Sublist,
        (a, b) if b.len() == 0 || a.windows(b.len()).any(|window| window == b) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}