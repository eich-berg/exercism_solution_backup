use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // todo!("How much of nucleotide type '{nucleotide}' is contained inside DNA string '{dna}'?");
    if !['A', 'C', 'G', 'T'].contains(&nucleotide) {
        return Err(nucleotide);
    }
    for c in dna.chars() {
        if !['A', 'C', 'G', 'T'].contains(&c) {
            return Err(c);
        }
    }
    Ok(dna.chars().filter(|c| *c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    //todo!("How much of every nucleotide type is contained inside DNA string '{dna}'?");
    let mut counts = HashMap::from([
        ('A', 0),
        ('C', 0),
        ('G', 0),
        ('T', 0),
    ]);
    for c in dna.chars() {
        match c {
            'A' | 'C' | 'G' | 'T' => *counts.entry(c).or_insert(0) += 1,
            invalid => return Err(invalid),
        }
    }
    Ok(counts)
}