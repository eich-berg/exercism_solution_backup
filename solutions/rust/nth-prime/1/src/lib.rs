pub fn nth(n: u32) -> u32 {
    // todo!("What is the 0-indexed {n}th prime number?")
    let mut primes = Vec::new();
    let mut i = 2;
    while primes.len() <= n as usize {
        if primes.iter().all(|&p| i % p != 0) { primes.push(i); }
        i += 1;
    }
    primes[n as usize]
}