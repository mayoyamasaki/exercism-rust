pub fn sum_of_squares(n: i64) -> i64 {
    (1..n+1).fold(0, |sum, i| sum + i.pow(2))
}

pub fn square_of_sum(n: i64) -> i64 {
    let s:i64 = (1..n+1).sum();
    s.pow(2)
}

pub fn difference(n: i64) -> i64 {
    square_of_sum(n) - sum_of_squares(n)
}
