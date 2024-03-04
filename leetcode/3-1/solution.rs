use std::collections::HashMap;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut unique = HashMap::new();

    for (idx, value) in nums.iter().enumerate() {
      let diff = target - value;
      
      if let Some(&prev_idx) = unique.get(&diff) {
        return vec![prev_idx as i32, idx as i32];
      }

      unique.insert(value, idx);
    }

    vec![]
  }
}
