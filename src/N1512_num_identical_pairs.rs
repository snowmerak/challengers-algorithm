use std::array;

use crate::Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut arr = [0; 101];
        for v in nums {
            arr[v as usize] += 1;
        }
        arr.iter().map(|f| { (f * (f-1))/2 }).collect::<Vec<i32>>().iter().sum()
    }
}