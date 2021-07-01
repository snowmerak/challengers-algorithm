use crate::Solution;

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let num = num.bytes().map(|x| {x as u8 - '0' as u8}).rev().collect::<Vec<u8>>();
        let mut cache = Vec::new();
        let mut state = 0;
        for i in num {
            if state == 0 && i % 2 == 1 {
                state = 1;
            }
            if state == 1 {
                cache.push(i);
            }
        }
        if cache.len() == 0 {
            return String::from("");
        }
        cache.reverse();
        return cache.iter().map(|f| {(f + '0' as u8) as char}).collect::<String>();
    }
}