pub fn abbreviate(msg: &str) -> String {
    let words = msg.split(|c: char| c.is_whitespace() || c == '-');
    let mut acronym  = String::new();
    for word in words {
        let mut prev_is_uppercase = false;
        for (i, c) in word.chars().enumerate() {
            if (i == 0 && c.is_alphabetic()) || (c.is_uppercase() && !prev_is_uppercase) {
                acronym.push(c);
            }
            is_prev_uppercase = c.is_uppercase();
        }
    }
    acronym.to_uppercase()
}
