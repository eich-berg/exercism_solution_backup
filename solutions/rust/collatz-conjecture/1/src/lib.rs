pub fn collatz(mut n: u64) -> Option<u64> {
    // todo!("return Some(x) where x is the number of steps required to reach 1 starting with {n}")
    let mut count = 0;
    if n == 0 { return None }
    while n > 1 {
        if n % 2 == 0 { n /= 2 } else { n = n*3 + 1 }
        count += 1;
    }
    Some(count) 
}
