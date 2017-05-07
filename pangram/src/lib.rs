use std::ascii::AsciiExt;
use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    const ALPHABET_NUM:usize = 26;
    let s:HashSet<char> = sentence.chars()
                          .filter(|c| c.is_ascii() && c.is_alphabetic())
                          .map(|c| c.to_ascii_lowercase())
                          .collect();
    s.len()  == ALPHABET_NUM
}
