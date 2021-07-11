use crate::Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut cache = [0; 101];
        for i in &nums {
            cache[*i as usize] += 1;
        }
        let mut result = Vec::new();
        for i in 0..nums.len() {
            let mut sum = 0;
            for v in 0..(nums[i] as usize) {
                sum += cache[v];
            }
            result.push(sum);
        }
        result
    }
}