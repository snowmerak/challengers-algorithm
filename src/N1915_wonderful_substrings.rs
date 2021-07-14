use crate::Solution;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut result = 0;
        let mut buffer = 1;
        let mut set = [false; 10];
        while buffer <= word.len() {
            for i in 0..word.len()-buffer {
                for v in i..i+buffer {
                    set[word.as_bytes()[v] as usize - 'a' as usize] = true;
                }
                if set.iter().filter(|b| **b).count() % 2 == 0 {
                    result += 1;
                }
                for i in 0..set.len() {
                    set[i] = false;
                }
            }
            buffer += 1;
        }
        result
    }
}