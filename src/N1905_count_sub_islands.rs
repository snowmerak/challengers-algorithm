use crate::Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let mut grid3 = Vec::new();
        for x in 0..grid1.len() {
            let mut xs = Vec::new();
            for y in 0..grid1[x].len() {
                xs.push(
                    match (grid1[x][y], grid2[x][y]) {
                        (0, 0) => 0,
                        (0, 1) => -1,
                        (1, 1) => 1,
                        (1, 0) => 0,
                        _ => 0,
                    }
                );
            }
            grid3.push(xs);
        }
        let mut queue = VecDeque::new();
        let mut total = 0;
        for x in 0..grid3.len() {
            for y in 0..grid3[x].len() {
                if grid3[x][y] == 1 {
                    queue.push_back((x, y));
                    let mut can = true;
                    while queue.len() > 0 {
                        let (x, y) = queue.pop_front().unwrap();
                        if x > 0 {
                            if grid3[x-1][y] == 1 || grid3[x-1][y] == -1 {
                                if grid3[x-1][y] == -1 {
                                    can = false;
                                }
                                let v = (x-1, y);
                                if !queue.contains(&v) {
                                    queue.push_back(v);
                                }
                            }
                        }
                        if x < grid1.len()-1 {
                            if grid3[x+1][y] == 1 || grid3[x+1][y] == -1 {
                                if grid3[x+1][y] == -1 {
                                    can = false;
                                }
                                let v = (x+1, y);
                                if !queue.contains(&v) {
                                    queue.push_back(v);
                                }
                            }
                        }
                        if y > 0 {
                            if grid3[x][y-1] == 1 || grid3[x][y-1] == -1 {
                                if grid3[x][y-1] == -1 {
                                    can = false;
                                }
                                let v = (x, y-1);
                                if !queue.contains(&v) {
                                    queue.push_back(v);
                                }
                            }
                        }
                        if y < grid1[x].len()-1 {
                            if grid3[x][y+1] == 1 || grid3[x][y+1] == -1 {
                                if grid3[x][y+1] == -1 {
                                    can = false;
                                }
                                let v = (x, y+1);
                                if !queue.contains(&v) {
                                    queue.push_back(v);
                                }
                            }
                        }
                        grid3[x][y] = 0;
                    }
                    if can {
                        total += 1;
                    }
                }
            }
        }
        return total;
    }
}