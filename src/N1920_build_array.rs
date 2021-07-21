use crate::Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr = Vec::new();
        arr.resize(nums.len(), 0);
        for i in 0..nums.len() {
            arr[i] = nums[nums[i] as usize];
        }
        arr
    }
}