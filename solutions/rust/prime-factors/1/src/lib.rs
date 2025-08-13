pub fn factors(mut n: u64) -> Vec<u64> {
    // todo!("This should calculate the prime factors of {n}")
    let mut i: u64 = 2;
    let mut factors: Vec<u64> = Vec::new();
    while n > 1 {
        if n % i == 0 {
            factors.push(i);
            n /= i;
        }
        else { i += 1; }
    }
    factors
}
