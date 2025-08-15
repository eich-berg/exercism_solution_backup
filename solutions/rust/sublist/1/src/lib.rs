#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    //todo!("Determine if the {first_list:?} is equal to, sublist of, superlist of or unequal to {second_list:?}.");
    if first_list == second_list { Comparison::Equal } 
    else if second_list.is_empty() || first_list.windows(second_list.len()).any(|window| window == second_list) 
    { 
        Comparison::Superlist
    } 
    else if first_list.is_empty() || second_list.windows(first_list.len()).any(|window| window == first_list) 
    { 
        Comparison::Sublist
    } 
    else { Comparison::Unequal }
}