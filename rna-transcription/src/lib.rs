#[derive(Debug,PartialEq)]
pub struct RibonucleicAcid {
    strand: String,
}

#[derive(Debug,PartialEq)]
pub struct DeoxyribonucleicAcid {
    strand: String,
}

impl RibonucleicAcid {
    pub fn new(rnas: &str) -> RibonucleicAcid {
        let mut strand = String::new();
        for c in rnas.chars() {
            match c {
                'A' => strand.push(c),
                'C' => strand.push(c),
                'G' => strand.push(c),
                'U' => strand.push(c),
                _ => panic!("Reqired A, C, G or U But found {}", c),
            }
        }
        RibonucleicAcid { strand: strand }
    }
}

impl DeoxyribonucleicAcid {
    pub fn new(dnas: &str) -> DeoxyribonucleicAcid {
        let mut strand = String::new();
        for c in dnas.chars() {
            match c {
                'A' => strand.push(c),
                'C' => strand.push(c),
                'G' => strand.push(c),
                'T' => strand.push(c),
                _ => panic!("Reqired A, C, G or T But found {}", c),
            }
        }
        DeoxyribonucleicAcid { strand: strand }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let mut strand = String::new();
        for c in self.strand.chars() {
            match c {
                'A' => strand.push('U'),
                'C' => strand.push('G'),
                'G' => strand.push('C'),
                'T' => strand.push('A'),
                _ => panic!("Reqired A, C, G or T But found {}", c),
            }
        }
        RibonucleicAcid::new(&strand)
    }
}
