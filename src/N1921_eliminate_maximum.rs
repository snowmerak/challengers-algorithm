use crate::Solution;

impl Solution {
    pub fn eliminate_maximum(mut dist: Vec<i32>, mut speed: Vec<i32>) -> i32 {
        let mut rs = 0;
        dist.reverse();
        speed.reverse();
        'outer: while dist.len() > 0 {
            dist.pop();
            rs += 1;
            println!("{:?}", dist);
            for i in 0..dist.len() {
                dist[i] = dist[i] - speed[i];
                if dist[i] < 0 {
                    break 'outer;
                }
            }
        }
        rs
    }
}