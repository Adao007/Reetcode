struct Solution; 
impl Solution {
    fn find_amount(nums: &Vec<i32>) -> i32 {
        let mut water = 0; 
        let mut left_ptr = 0; 
        let mut right_ptr = nums.len() - 1; 

        while left_ptr < right_ptr {
            let height = nums[left_ptr].min(nums[right_ptr]); 
            let length = (right_ptr - left_ptr) as i32;
            water = water.max(height * length);

            if nums[left_ptr] < nums[right_ptr] {
                left_ptr += 1; 
            }
            else {
                right_ptr -= 1;
            }
        } 

        water
    }
}

fn main() {
    let mut heights = vec![1, 7, 2, 5, 4, 7, 3, 6]; 
    println!("{:?}", Solution::find_amount(&heights));
    heights = vec![2, 2, 2];
    println!("{:?}", Solution::find_amount(&heights));
    heights = vec![0, 1, 2, 3, 4, 5, 6, 7]; 
    println!("{:?}", Solution::find_amount(&heights));
}
