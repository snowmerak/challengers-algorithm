use crate::Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_common_subpath(_n: i32, paths: Vec<Vec<i32>>) -> i32 {
        let bound = 214723;
        let x = 10001;
        let mut map = Vec::new();
        map.resize(paths.len(), Vec::new());
        for i in 0..map.len() {
            map[i].resize(paths[i].len(), 0);
        }
        let mut max = 0;
        let shortest_length = paths.iter().map(|x| x.len()).min().unwrap();
        for buf in 0..shortest_length {
            for i in 0..paths.len() {
                for j in 0..paths[i].len()-buf {
                    map[i][j] = (map[i][j] * x + (paths[i][j+buf]+1)) % bound;
                }
                for j in paths[i].len()-buf..paths[i].len() {
                    map[i][j] = -1;
                }
            }
            // println!("{:?}", map);
            let sets = map.iter().map(|x| x.iter().collect::<HashSet<_>>()).collect::<Vec<HashSet<_>>>();
            // println!("{:?}", sets);
            let count = sets[0].iter().filter(|x| sets.iter().all(|k| k.contains(*x))).count();
            // println!("{:?}", sets[0].iter().filter(|x| sets.iter().all(|k| k.contains(*x))).collect::<HashSet<_>>());
            // println!("{:?}", count);
            match buf {
                0 => {
                    if count > 0 {
                        max = buf+1;
                    }
                }
                _ => {
                    if count > 1 {
                        max = buf+1;
                    }
                }
            }
        }
        max as i32
    }
}