use std::collections::HashSet;

pub struct Brackets {
    s: String
}

impl<'a> From<&'a str> for Brackets {
    fn from(s: &'a str) -> Brackets {
        Brackets {s: s.to_string() }
    }
}

impl Brackets {
    pub fn are_balanced(&self) -> bool {
        let lhb: HashSet<_> = ['(', '{', '['].iter().cloned().collect();
        let mut q: Vec<char> = Vec::new();
        for c in self.s.chars() {
            if lhb.contains(&c) {
                q.push(c);
            } else {
                let expect = match c {
                    ')' => '(',
                    '}' => '{',
                    ']' => '[',
                    _ => continue
                };
                if let Some(real) = q.pop() {
                    if real != expect {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        q.len() == 0
    }
}

