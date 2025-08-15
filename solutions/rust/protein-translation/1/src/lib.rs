pub fn translate(rna: &str) -> Option<Vec<&str>> {
    // todo!("Return a list of protein names that correspond to the '{rna}' RNA string or None if the RNA string is invalid");
    // Convert to Vec<char> to use windows()
    if rna.len() % 3 != 0 && !rna.contains("UAA") && !rna.contains("UAG") && !rna.contains("UGA") {
        return None;
    }
    let mut proteins = Vec::new();
    let rna_vec: Vec<char> = rna.chars().collect();
    for codon_chunk in rna_vec.chunks_exact(3) {
        let protein = match codon_chunk {
            ['A', 'U', 'G'] => "Methionine",
            ['U', 'U', 'U'] | ['U', 'U', 'C'] => "Phenylalanine",
            ['U', 'U', 'A'] | ['U', 'U', 'G'] => "Leucine",
            ['U', 'C', 'U'] | ['U', 'C', 'C'] | ['U', 'C', 'A'] | ['U', 'C', 'G'] => "Serine",
            ['U', 'A', 'U'] | ['U', 'A', 'C'] => "Tyrosine",
            ['U', 'G', 'U'] | ['U', 'G', 'C'] => "Cysteine",
            ['U', 'G', 'G'] => "Tryptophan",
            ['U', 'A', 'A'] | ['U', 'A', 'G'] | ['U', 'G', 'A'] => return Some(proteins),
            _ => return None, // Invalid codon
    };
        proteins.push(protein);
    }
    Some(proteins)
}