pub fn raindrops(n: u32) -> String {
    //todo!("what sound does Raindrop #{n} make?")
    let mut raindrops = String::new();
    if n%3 == 0 { raindrops.push_str("Pling") }
    if n%5 == 0 { raindrops.push_str("Plang") }
    if n%7 == 0 { raindrops.push_str("Plong") }
    if (n%3 != 0) && (n%5 != 0) && (n%7 != 0) {
        raindrops.push_str(&n.to_string()) 
    }
    raindrops
}
