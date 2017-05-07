use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::iter::repeat;

const NUCLEOTIDES: [char; 4] = ['G', 'C', 'T', 'A'];

pub fn count(symbol: char, dna: &str) -> Result<usize, DNAError> {
    let nucleotides: HashSet<char> = NUCLEOTIDES.iter()
                                                .cloned()
                                                .collect();
    if dna.chars().filter(|c| !nucleotides.contains(c)).count() > 0 ||
       !nucleotides.contains(&symbol) {
        Err(DNAError::IlegalSymbol)
    } else {
        Ok(dna.matches(symbol).count())
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, DNAError> {
    let mut dist: HashMap<char, usize> = NUCLEOTIDES.iter()
                                                    .cloned()
                                                    .zip(repeat(0))
                                                    .into_iter()
                                                    .collect();

    for c in dna.chars() {
        if let Some(v) = dist.get_mut(&c) {
            *v += 1;
        } else {
            return Err(DNAError::IlegalSymbol);
        }
    }

    Ok(dist)
}

#[derive(Debug)]
pub enum DNAError {
    IlegalSymbol,
}

impl fmt::Display for DNAError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DNAError::IlegalSymbol => write!(f, "Required symbolds is 'A', 'C', 'G', or 'T'")
        }
    }
}

impl Error for DNAError {
    fn description(&self) -> &str {
        match *self {
            DNAError::IlegalSymbol => "Found ilegal symbol"
        }
    }
}
