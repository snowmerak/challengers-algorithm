use crate::Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut a = (0, 1, 1);
        for _ in 0..n {
            let _0 = a.0;
            a.0 = a.1;
            a.1 = a.2;
            a.2 = _0 + a.0 + a.1;
        }
        a.0
    }
}