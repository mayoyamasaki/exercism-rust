use std::ascii::AsciiExt;

pub fn encode(msg: &str) -> String {
    msg.to_lowercase()
       .chars()
       .filter(|c| c.is_alphanumeric() && c.is_ascii())
       .map(sub)
       .collect::<Vec<char>>()
       .chunks(5)
       .map(|c| c.iter().cloned().collect())
       .collect::<Vec<String>>()
       .join(" ")
}

pub fn decode(code: &str) -> String {
    code.chars()
        .filter(|c| c.is_alphanumeric() && c.is_ascii())
        .map(sub)
        .collect()
}

fn sub(c: char) -> char {
    if c.is_numeric() {
        c
    } else {
        ('z' as u8 + 'a' as u8 - c as u8) as char
    }
}
