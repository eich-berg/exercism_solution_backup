pub fn factors(mut n: u64) -> Vec<u64> {
    //todo!("This should calculate the prime factors of {n}")
    let mut prime_factors = Vec::new();
    let mut i = 2;
    while n > 1 {
        if n % i == 0 { 
            prime_factors.push(i);
            n /= i;
        }
        else { i += 1; }    
    }
    prime_factors 
}