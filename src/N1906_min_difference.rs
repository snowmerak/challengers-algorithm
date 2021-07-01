use crate::Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut rs = Vec::new();

        for q in &queries {
            let mut set = BTreeSet::new();
            for i in q[0]..=q[1] {
                set.insert(nums.get(i as usize).unwrap());
            }
            if set.len() == 0 {
                rs.push(-1);
                continue;
            }
            let mut min = 101;
            let mut prev = 200;
            for x in set {
                let a = (*x - prev).abs();
                if a == 0 {
                    continue;
                }
                if min > a {
                    min = a;
                }
                prev = *x;
            }
            min = if min == 101 {
                -1
            } else {
                min
            };
            rs.push(min);
        }

        return rs;
    }
}