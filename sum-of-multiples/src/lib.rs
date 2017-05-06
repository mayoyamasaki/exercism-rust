pub fn sum_of_multiples2(n: i64, nums: &Vec<i64>) -> i64 {
    let mut result = 0;
    for i in 1..n {
        for j in nums {
            if i % j == 0 {
                result += i;
                break;
            }
        }
    }
    result
}


pub fn sum_of_multiples(n: i64, nums: &Vec<i64>) -> i64 {
    let is_multiple = |i: &i64| nums.iter().any(|&n| i % n == 0);
    (1..n).filter(is_multiple).sum()
}
