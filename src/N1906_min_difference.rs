use crate::Solution;

impl Solution {
    pub fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut rs = Vec::new();

        for q in &queries {
            let (s, e) = (q[0] as usize, q[1] as usize);
            let mut a = Vec::new();
            for i in s..=e {
                match a.binary_search(&nums[i]) {
                    Ok(_) => continue,
                    Err(e) => a.insert(e, nums[i]),
                }
            }
            if a.len() < 2 {
                rs.push(-1);
                continue;
            }
            let mut min = 101;
            for i in 0..a.len()-1 {
                let t = a[i+1] - a[i];
                if t < min {
                    min = t;
                }
            }
            rs.push(min);
        }

        return rs;
    }
}