use crate::Solution;

impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        let last = nums.len()-1;
        (nums[last] * nums[last-1]) - (nums[1] * nums[0])
    }
}