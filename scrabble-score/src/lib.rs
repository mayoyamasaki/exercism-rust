use std::ascii::AsciiExt;

fn point(c: char) -> u32 {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
        'd' | 'g' => 2,
        'b' | 'c' | 'm' | 'p' => 3,
        'f' | 'h' | 'v' | 'w' | 'y' => 4,
        'k' => 5,
        'j' | 'x' => 8,
        'q' | 'z' => 10,
        _ => 0,
    }
}

pub fn score(letter: &str) -> u32 {
    letter.chars().map(point).sum()
}
