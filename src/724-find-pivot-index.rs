use std::convert::TryFrom;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut total_sum = 0;
        
        for val in &nums {
            total_sum += val;
        }
        
        let mut left_sum = 0;
        let mut right_sum = total_sum;
        
        for i in 0..nums.len() {
            if (i != 0) {
                left_sum += nums[i-1];
            }
            
            right_sum -= nums[i];
            
            if (right_sum == left_sum) {
                return i32::try_from(i).ok().unwrap();
            }
        }
        
        return -1;
    }
}
