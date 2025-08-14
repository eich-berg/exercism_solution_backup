pub fn nth(n: usize) -> u32 {
    //todo!("What is the 0-indexed {n}th prime number?")
    let mut primes = vec![2];
    let mut candidate = 3; 
    while primes.len() <= n {
        if primes.iter().all(|&p| candidate % p != 0) {
            primes.push(candidate);
        }
        candidate += 2; // Skip even numbers
    }
    primes[n]
}
