use std::iter::repeat;

pub fn encode(data: &str) -> String {
    let mut data = data.chars();
    let mut encoded = String::new();

    let mut count: usize = 1;
    let mut prev_c = match data.next() {
        Some(c) => c,
        None => return encoded
    };

    let fmt = |n, c| {
        match n {
            1 => format!("{}", c),
            _ => format!("{}{}", n, c)
        }
    };

    for c in data {
        if c != prev_c {
            encoded.push_str(&fmt(count, prev_c));
            prev_c = c;
            count = 0;
        }
        count += 1;
    }
    encoded.push_str(&fmt(count, prev_c));

    encoded
}

pub fn decode(encoded: &str) -> String {
    let mut data = String::new();
    let mut count:usize = 0;

    for c in encoded.chars() {
        if c.is_numeric() {
            let n = c.to_digit(10).unwrap() as usize;
            count = count * 10 + n;
        } else {
            if count == 0 { count = 1; }
            let repeated: String = repeat(c).take(count).collect();
            data.push_str(&repeated);
            count = 0;
        }
    }

    data
}
