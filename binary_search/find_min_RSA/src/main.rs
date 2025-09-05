struct Solution; 
impl Solution {
    fn get_min(nums: Vec<i32>) -> i32 {
        let mut min = 1001; // Problem list's nums value range as -1000 <= nums[i] <= 1000
        let mut left_ptr = 0; 
        let mut right_ptr = nums.len() - 1; 
        let mut mid = ((right_ptr - left_ptr) / 2) + left_ptr; 

        while left_ptr < right_ptr {
            if nums[left_ptr] < nums[right_ptr] { 
                return nums[left_ptr]; 
            }

            if nums[mid] > nums[right_ptr] {
                left_ptr = mid + 1; 
            }
            else {
                right_ptr = mid - 1; 
            }

            mid = ((right_ptr - left_ptr) / 2) + left_ptr;
            min = min.min(nums[mid]);
        }

        min
    }
}

fn main() {
    let example1 = vec![3, 4, 5, 6, 1, 2]; 
    let example2 = vec![1, 2, 3, 4, 5, 0];
    let example3 = vec![4, 5, 6, 7];
    
    println!("The minimum value in Example 1 is {}", Solution::get_min(example1));
    println!("The minimum value in Example 2 is {}", Solution::get_min(example2));
    println!("The minimum value in Example 3 is {}", Solution::get_min(example3));
}
