use std::collections::HashSet;
use std::fmt;
use std::error::Error;

pub struct Triangle {
    usides: HashSet<u32>,
}

impl Triangle {
    pub fn build(sides: [u32; 3]) -> Result<Triangle, TriangleError> {
        let maxside = sides.iter().cloned().max().unwrap();
        let sumside:u32 = sides.iter().sum();
        if sumside <= 2*maxside {
            return Err(TriangleError::FailedToConstruct);
        }

        let usides: HashSet<u32> = sides.iter().cloned().collect();
        Ok(Triangle { usides: usides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.usides.len() == 1
    }

    pub fn is_isosceles(&self) -> bool {
        self.usides.len() == 2
    }

    pub fn is_scalene(&self) -> bool {
        self.usides.len() == 3
    }
}

#[derive(Debug)]
pub enum TriangleError {
    FailedToConstruct,
}

impl fmt::Display for TriangleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TriangleError::FailedToConstruct => write!(f, "Failed to construct traiangle"),
        }
    }
}

impl Error for TriangleError {
    fn description(&self) -> &str {
        match *self {
            TriangleError::FailedToConstruct => "Failed to construct a new triagnle",
        }
    }
}
