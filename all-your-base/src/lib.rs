///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
pub fn convert(numbers: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, ()> {
    match to10(&numbers, from_base) {
        Ok(base10) => to_n(base10, to_base),
        Err(e) => Err(e)
    }
}

fn to10(numbers: &[u32], from_base: u32) -> Result<u32, ()> {
    if from_base < 2 { return Err(()); }

    let mut weight = 1;
    let mut result = 0;
    for &n in numbers.iter().rev() {
        if n >= from_base {
            return Err(())
        }
        result += n * weight;
        weight *= from_base;
    }
    Ok(result)
}

fn to_n(number: u32, to_base: u32) -> Result<Vec<u32>, ()> {
    if to_base < 2 { return Err(()); }

    let mut number = number;
    let mut v = Vec::new();
    while number >= to_base {
        v.push(number % to_base);
        number = number / to_base;
    }

    if !(v.len() == 0 && number == 0) {
        v.push(number);
    }
    v.reverse();

    Ok(v)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_to10() {
        assert_eq!(5, to10(&[1,0,1], 2).unwrap());
    }

    #[test]
    fn overflow_value_to10() {
        assert!(to10(&[1,2,1], 2).is_err());
    }

    #[test]
    fn zero_base_to10() {
        assert!(to10(&[], 0).is_err());
    }

    #[test]
    fn to_base_two() {
        assert_eq!(&[1,0,1,0], to_n(10, 2).unwrap().as_slice());
    }

    #[test]
    fn to_base_one() {
        assert!(to_n(10, 1).is_err());
    }
}
