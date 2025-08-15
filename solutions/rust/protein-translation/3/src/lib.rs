pub fn translate(rna: &str) -> Option<Vec<&str>> {
    // todo!("Return a list of protein names that correspond to the '{rna}' RNA string or None if the RNA string is invalid");
    if rna.len() % 3 != 0 && !rna.contains("UAA") && !rna.contains("UAG") && !rna.contains("UGA") {
        return None;
    }
    let mut proteins = Vec::new();
    let as_bytes = rna.as_bytes(); // Get UTF-8 bytes
    for codon_chunk in as_bytes.chunks_exact(3) {
        let protein = match codon_chunk {
            b"AUG" => "Methionine",
            b"UUU" | b"UUC" => "Phenylalanine",
            b"UUA" | b"UUG" => "Leucine",
            b"UCU" | b"UCC" | b"UCA" | b"UCG" => "Serine",
            b"UAU" | b"UAC" => "Tyrosine",
            b"UGU" | b"UGC" => "Cysteine",
            b"UGG" => "Tryptophan",
            b"UAA" | b"UAG" | b"UGA" => return Some(proteins),
            _ => return None, // Invalid codon
    };
        proteins.push(protein);
    }
    Some(proteins)
}