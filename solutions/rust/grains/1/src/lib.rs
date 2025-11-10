pub fn square(s: u32) -> u64 {
    // todo!("grains of rice on square {s}");
    assert!(s >= 1, "Square must be between 1 and 64");
    (2_u128.pow(s)/2).try_into().unwrap()
}

pub fn total() -> u64 {
    (2_u128.pow(64)-1).try_into().unwrap()
}
