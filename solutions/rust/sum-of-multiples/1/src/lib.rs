use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // todo!("Sum the multiples of all of {factors:?} which are less than {limit}")
    let mut multiples = Vec::new();
    for factor in factors {
        if *factor == 0 { continue; }
        multiples.extend((*factor..limit).step_by((*factor).try_into().unwrap()));
    }
    multiples.into_iter().collect::<HashSet<_>>().into_iter().sum()
}


