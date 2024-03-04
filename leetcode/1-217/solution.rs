use std::collections::HashSet;

impl Solution {
  pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut unique: HashSet<i32> = HashSet::with_capacity(nums.len());

    for n in nums {
      if !unique.insert(n) {
        return true;
      }
    }

    false
  }
}
