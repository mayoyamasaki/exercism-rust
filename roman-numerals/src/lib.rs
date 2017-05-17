use std::fmt;

static ARABIC_ROMAN: [(u32, &'static str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

pub struct Roman {
    numerals: String
}

impl From<u32> for Roman {
    fn from(n: u32) -> Roman {
        let mut n = n;
        let mut numerals = String::new();
        for &(v, s) in ARABIC_ROMAN.iter() {
            while n >= v {
                n -= v;
                numerals.push_str(s)
            }
        }

        Roman { numerals: numerals }
    }
}

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.numerals)
    }
}
