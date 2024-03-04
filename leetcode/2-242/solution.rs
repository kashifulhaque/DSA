use std::collections::HashMap;

impl Solution {
  pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
      return false;
    }

    let mut s_count = [0; 26];
    let mut t_count = [0; 26];

    for c in s.chars() {
      s_count[c as usize - 'a' as usize] += 1;
    }

    for c in t.chars() {
      t_count[c as usize - 'a' as usize] += 1;
    }

    s_count == t_count
  }
}
