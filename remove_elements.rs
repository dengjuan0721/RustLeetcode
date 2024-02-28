struct Solution {}

impl Solution {
    pub fn new() -> Solution{
        Solution{}
    } 
     
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut j:usize = 0; //写指针
        let mut count:usize = 0;
        for i in 0..nums.len() { //读指针
            //双指针
            if nums[i] == val {
                count += 1;
                continue; //i动j不动
            }else{
                nums[j] = nums[i];
                j += 1;
            }
        }
        (nums.len()-count) as i32
    }

}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let new_len = Solution::remove_element(&mut nums, val);
    println!("New length: {}", new_len);
    println!("Updated array: {:?}", nums);
}
