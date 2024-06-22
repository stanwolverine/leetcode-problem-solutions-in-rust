use std::collections::HashMap;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    // Using into_iter instead of iter
    // so that we get exclusive values (i32) instead of shared values (&i32)
    for (index, num) in nums.into_iter().enumerate() {
      let diff = target - num;
      if let Some(idx) = map.get(&diff) {
        return vec![index as i32, idx.to_owned() as i32]
      } else {
        map.insert(num, index);
      }
    }

    // In case `nums` is empty vector or none of the items sums upto `target`
    // Which will not happen, but compiler does not know that
    return vec![0, 1]
  }
}
