struct Solution; 
impl Solution {
    fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left_pointer = 0; 
        let mut right_pointer = nums.len() - 1; 

        while left_pointer < right_pointer {
            let mid = (right_pointer - left_pointer / 2) + left_pointer; 
            if nums[mid] < target {
                left_pointer = mid as usize + 1; 
            }
            else if nums[mid] > target {
                right_pointer = mid as usize - 1; 
            }
            else {
                return mid as i32;
            }
        }

        -1
    }
}

fn main() {
    let nums = vec![-1, 0, 2, 4, 6, 8]; 
    let target = 3; // Also try 4!

    assert_eq!(-1, Solution::binary_search(nums, target)); 
    println!("All good! The correct index was returned!"); 
}
