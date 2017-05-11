use std::ascii::AsciiExt;

pub fn rotate(msg: &str, n:u8) -> String {
    let n:u8 = n % 26;
    msg.chars().map(|c| rot_n(c, n)).collect()
}

fn rot_n(c: char, n: u8) -> char {
    if !c.is_alphabetic() || !c.is_ascii() { return c; }
    let base = if c.is_uppercase() { 'A' as u8 } else { 'a' as u8 };
    ((c as u8 - base + n) % 26 + base) as char
}
