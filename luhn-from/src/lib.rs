pub struct Luhn {
    digits: String,
}

impl<'a> From<&'a str> for Luhn {
    fn from(s: &'a str) -> Luhn {
        Luhn { digits: s.to_string() }
    }
}

impl From<String> for Luhn {
    fn from(s: String) -> Luhn {
        Luhn { digits: s}
    }
}

impl From<u8> for Luhn {
    fn from(n: u8) -> Luhn {
        Luhn { digits: n.to_string() }
    }
}

impl From<u16> for Luhn {
    fn from(n: u16) -> Luhn {
        Luhn { digits: n.to_string() }
    }
}

impl From<u32> for Luhn {
    fn from(n: u32) -> Luhn {
        Luhn { digits: n.to_string() }
    }
}

impl From<u64> for Luhn {
    fn from(n: u64) -> Luhn {
        Luhn { digits: n.to_string() }
    }
}

impl From<usize> for Luhn {
    fn from(n: usize) -> Luhn {
        Luhn { digits: n.to_string() }
    }
}

impl Luhn {
    pub fn is_valid(&self) -> bool {

        let mut sum: u32 = 0;
        let digits:String = self.digits.chars()
                                       .filter(|c| !c.is_whitespace())
                                       .collect();
        if digits.chars().count() < 2 { return false; }

        for (i, c) in digits.chars().rev().enumerate() {
            if let Some(n) = c.to_digit(10) {
                if i % 2 == 0 {
                    sum += n;
                } else {
                    sum += n*2 / 10 + n*2 % 10;
                }
            } else {
                return false;
            }
        }
        sum % 10 == 0
    }
}
