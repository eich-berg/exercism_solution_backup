use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    //todo!("How much of nucleotide type '{nucleotide}' is contained inside DNA string '{dna}'?");
    if !"ACGT".contains(nucleotide) { return Err(nucleotide); }
    let mut count = 0;
    for c in dna.chars() { 
        if !"ACTG".contains(c) { return Err(c); }
        if c == nucleotide { count += 1; }
    }
    Ok(count)
}


pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    //todo!("How much of every nucleotide type is contained inside DNA string '{dna}'?");
    let mut nucleotide_counts = HashMap::from([('G', 0), ('C', 0), ('T', 0), ('A', 0)]);
    for c in dna.chars() {
        if !"ACGT".contains(c) { return Err(c); }
        *nucleotide_counts.get_mut(&c).unwrap() += 1;
    }
    Ok(nucleotide_counts)
}