pub fn is_valid(digits: &str) -> bool {
    let digits:String = digits.chars()
                              .filter(|c| !c.is_whitespace())
                              .collect();

    if digits.chars().count() < 2 { return false; }

    let mut sum: u32 = 0;
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
