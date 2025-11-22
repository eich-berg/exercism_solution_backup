use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna { input: String }

#[derive(Debug, PartialEq, Eq)]
pub struct Rna { output: String }

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        // todo!("Construct new Dna from '{dna}' string. If string contains invalid nucleotides return index of first invalid nucleotide");
        for (i, c) in dna.chars().enumerate() { if !"GCTA".contains(c) { return Err(i); } } 
        Ok( Dna { input: dna.to_string() } )
    }
    pub fn into_rna(self) -> Rna {
        // todo!("Transform Dna {self:?} into corresponding Rna");
        let lookup = HashMap::from([('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')]);
        let output: String = self.input.chars().map(|c| lookup.get(&c).unwrap()).collect();
        Rna { output }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        // todo!( "Construct new Rna from '{rna}' string. If string contains invalid nucleotides return index of first invalid nucleotide");
        for (i, c) in rna.chars().enumerate() { if !"GCAU".contains(c) { return Err(i); } } 
        Ok( Rna { output: rna.to_string() } )
    }
}