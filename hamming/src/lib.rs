use std::fmt;
use std::error::Error;


#[derive(Debug)]
pub enum HammingDistanceError {
    NotEqualLen,
}

impl fmt::Display for HammingDistanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HammingDistanceError::NotEqualLen => write!(f, "Required same length")
        }
    }
}

impl Error for HammingDistanceError {
    fn description(&self) -> &str {
        match *self {
            HammingDistanceError::NotEqualLen => "not equal length"
        }
    }
}

pub fn hamming_distance(a: &str, b: &str) -> Result<usize, HammingDistanceError> {
    if a.len() != b.len() {
        return Err(HammingDistanceError::NotEqualLen)
    }

    Ok(a.chars().zip(b.chars())
                .filter(|&(i, j)| i != j)
                .count())
}
