extern crate num_traits;

use std::error::Error;
use std::fmt;
use std::iter::Sum;
use num_traits::{Num, NumCast};

pub struct Triangle {
    num_uniq_sides: u32,
}

impl Triangle {
    pub fn build<T>(sides: [T; 3]) -> Result<Triangle, TriangleError>
        where T: Num + NumCast + Copy + Sum + PartialOrd {
        let maxside: T = sides.iter()
                              .cloned()
                              .fold(sides[0], |m, v| if v > m {v} else {m});
        let sumside:T = sides.iter().cloned().sum();
        // Triagnle requires a + b > c where a < c and b < c.
        // In other word, it's (a + b + c - c) > c
        if (sumside - maxside) <=  maxside {
            return Err(TriangleError::FailedToConstruct);
        }

        let mut num_uniq_sides = 3;
        if sides[0] == sides[1] { num_uniq_sides -= 1; }
        if sides[1] == sides[2] { num_uniq_sides -= 1; }
        if sides[2] == sides[0] { num_uniq_sides -= 1; }

        Ok(Triangle { num_uniq_sides: num_uniq_sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.num_uniq_sides <= 1
    }

    pub fn is_isosceles(&self) -> bool {
        self.num_uniq_sides == 2
    }

    pub fn is_scalene(&self) -> bool {
        self.num_uniq_sides == 3
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
