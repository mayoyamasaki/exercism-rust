extern crate num_traits;

use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use num_traits::{Num, NumCast};

pub struct Triangle<T> {
    uniq_sides: Vec<T>,
}

impl<T> Triangle<T>
    where T: Num + NumCast + Copy + PartialOrd {

    pub fn build(sides: [T; 3]) -> Result<Triangle<T>, TriangleError> {
        let mut sides: Vec<T> = sides.iter().cloned().collect();
        sides.sort_by(|a, b| if a < b { Ordering::Less } else { Ordering::Greater });

        if sides[0] + sides[1] <= sides[2] {
            return Err(TriangleError::FailedToConstruct);
        }

        let mut uniq_sides: Vec<T> = Vec::new();
        for side in sides.into_iter() {
            if let Some(last_side) = uniq_sides.last() {
                if *last_side == side {
                    continue;
                }
            }
            uniq_sides.push(side);
        }

        Ok(Triangle { uniq_sides: uniq_sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.uniq_sides.len() == 1
    }

    pub fn is_isosceles(&self) -> bool {
        self.uniq_sides.len() == 2
    }

    pub fn is_scalene(&self) -> bool {
        self.uniq_sides.len() == 3
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
