struct Solution {}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable(); // 更高效的排序
        let mut ans: Vec<Vec<i32>> = Vec::new();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] { // 跳过重复值
                continue;
            }
            let mut l = i + 1; // 左指针
            let mut r = nums.len() - 1; // 右指针
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum == 0 {
                    ans.push(vec![nums[i], nums[l], nums[r]]);
                    while l < r && nums[l] == nums[l + 1] { l += 1; } // 跳过重复值
                    while l < r && nums[r] == nums[r - 1] { r -= 1; } // 跳过重复值
                    l += 1;
                    r -= 1;
                } else if sum < 0 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        ans
    }
}

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4]; //test
    let sol = Solution::three_sum(nums);
    println!("{:?}", sol);
}
