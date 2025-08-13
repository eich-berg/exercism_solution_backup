pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    //todo!("Sum the multiples of all of {factors:?} which are less than {limit}")
    let mut mult_set = Vec::new();
    for fac in factors {
        if *fac == 0 { continue; }
        mult_set.extend((1..).map(|i| fac*i).take_while(|x| *x < limit));
    }
    mult_set.sort();
    mult_set.dedup();
    mult_set.into_iter().sum()
}