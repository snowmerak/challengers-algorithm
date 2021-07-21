use crate::Solution;

impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        let odds = 4;
        let evens = 5;
        let bound = 1000000007;
        let mut result = 1 as i64;
        for i in 0..n {
            match i % 2 {
                0 => {
                    result *= evens
                }
                1 => {
                    result *= odds
                }
                _ => {
                    //"이 경우가 어케 나오는데!"
                }
            }
            result %= bound
        }
        result as i32
    }
}