use crate::Solution;

impl Solution {
    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut new_grid = grid.clone();
        let mut top = 0 as usize;
        let mut bot = grid.len() - 1 as usize;
        let mut left = 0 as usize;
        let mut right = grid[0].len() - 1 as usize;
        while top < bot && left < right {
            let mut pos = (top, right);
            let step = k % ((bot-top) * 2 + (right-left) * 2);
            loop {
                let mut moved = pos.clone();
                for _ in 0..step {
                    if moved.0 == top {
                        if moved.1 == left {
                            moved.0 += 1;
                        } else {
                            moved.1 -= 1;
                        }
                    } else if moved.0 == bot {
                        if moved.1 == right {
                            moved.0 -= 1;
                        } else {
                            moved.1 += 1;
                        }
                    } else {
                        if moved.1 == left {
                            moved.0 += 1;
                        } else if moved.1 == right {
                            moved.0 -= 1;
                        }
                    }
                }
                new_grid[moved.0][moved.1] = grid[pos.0][pos.1];
                if pos.0 == top {
                    if pos.1 == left {
                        pos.0 += 1;
                    } else {
                        pos.1 -= 1;
                    }
                } else if pos.0 == bot {
                    if pos.1 == right {
                        pos.0 -= 1;
                    } else {
                        pos.1 += 1;
                    }
                } else if pos.0 == bot {
                    pos.1 += 1;
                } else {
                    if pos.1 == left {
                        pos.0 += 1;
                    } else if pos.1 == right {
                        pos.0 -= 1;
                    }
                }
                if pos.0 == top && pos.1 == right {
                    break;
                }
            }
            top += 1;
            bot -= 1;
            left += 1;
            right -= 1;
        }
        new_grid
    }
}