pub fn collatz(mut n: u64) -> Option<u64> {
    // todo!("return Some(x) where x is the number of steps required to reach 1 starting with {n}")
    let mut counter = 0;
    if n < 1 { return None; }
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
        }
        else {
            n = n*3 + 1;
        }
        counter += 1;
    }
    Some(counter)
}
