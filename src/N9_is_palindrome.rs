use crate::Solution;

impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
      if x < 0 {
        false
      } else {
        let x_str = format!("{}", x);
        let x_bytes = x_str.as_bytes();
        for i in 0..x_bytes.len()/2 {
          if x_bytes.get(i).unwrap() != x_bytes.get(x_bytes.len() - 1 - i).unwrap() {
            return false
          }
        }
        true
      }
  }
}