use std::{cmp, fmt};
use std::error::Error;

pub fn lsp(digits: &str, n: usize) -> Result<u32, LSPError> {
    if n == 0 { return Ok(1); }
    if digits.len() < n { return Err(LSPError::IllegalLength); }

    let digits = match str2nums(digits) {
        Ok(nums) => nums,
        Err(err) => return Err(err),
    };

    let mut max = 0;
    for window in digits.windows(n) {
        let v = window.iter().product();
        max = cmp::max(v, max);
    }

    Ok(max)
}

fn str2nums(s: &str) -> Result<Vec<u32>, LSPError> {
    let mut nums:Vec<u32> = Vec::new();
    for c in s.chars() {
        if let Some(n) = c.to_digit(10) {
            nums.push(n);
        } else {
            return Err(LSPError::IllegalToken);
        }
    }
    return Ok(nums);
}

#[derive(Debug)]
pub enum LSPError {
    IllegalLength,
    IllegalToken,
}

impl fmt::Display for LSPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LSPError::IllegalLength => write!(f, "Required str.len() >= number"),
            LSPError::IllegalToken => write!(f, "Required digit chars"),
        }
    }
}

impl Error for LSPError {
    fn description(&self) -> &str {
        match *self {
            LSPError::IllegalLength => "Found illegal length",
            LSPError::IllegalToken => "Found illegal token",
        }
    }
}
