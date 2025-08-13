pub fn raindrops(n: u32) -> String {
    // todo!("what sound does Raindrop #{n} make?")
    let mut sounds = String::new();
    if n % 3 == 0 { sounds += "Pling"; } 
    if n % 5 == 0 { sounds += "Plang"; } 
    if n % 7 == 0 { sounds += "Plong"; } 
    if sounds.is_empty() { n.to_string() } else { sounds }
}