use crate::Solution;

impl Solution {
    pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        let mut count = Vec::new();
        count.resize(target.len(), 0);
        for v in arr {
            for i in 0..count.len() {
                if target.len() < i + count[i] {
                    continue;
                }
                if v == target[i+count[i]] {
                    count[i] += 1;
                }
            }
        }
        println!("{:?}", &count);
        0
    }
}