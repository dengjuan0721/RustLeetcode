//version 1,running out of time
use std::cmp::min;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut start_index = 0;
        let mut ans = nums.len();
        while start_index < nums.len() {
            let mut sum = 0;
            if nums[start_index] >= target {return 1};
            for i in start_index..nums.len() { //左闭右开
                sum += nums[i];
                if sum >= target{
                    ans = min(i-start_index+1, ans);
                    break;
                }
                if i == nums.len()-1 && start_index == 0 {
                    return 0;
                }
            }
            start_index += 1;
        }
        return ans as i32;
    }
}
