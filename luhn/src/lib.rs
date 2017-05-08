pub fn is_valid(digits: &str) -> bool {

    let mut sum: u32 = 0;
    let digits:String = digits.chars().filter(|c| !c.is_whitespace()).collect();

    if digits.chars().count() < 2 { return false; }

    for (i, c) in digits.chars().rev().enumerate() {
        println!("{}", c);
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
    println!("{}", sum);
    sum % 10 == 0
}
