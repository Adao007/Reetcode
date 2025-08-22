use std::ptr::read_unaligned;

struct Solution; 
impl Solution {
    fn trap_rain(height: &Vec<i32>) -> i32 {
        let mut rain = 0; 
        let mut left_ptr = 1; let mut right_ptr = height.len() - 1; 
        let mut left_max = height[left_ptr]; let mut right_max = height[right_ptr]; 

        while left_ptr < right_ptr {
            if left_max < right_max {
                left_ptr += 1;
                left_max = left_max.max(height[left_ptr]); 
                rain += left_max - height[left_ptr];
            }
            else { 
                right_ptr -= 1; 
                right_max = right_max.max(height[right_ptr]); 
                rain += right_max - height[right_ptr];
            }
        }

        rain
    }
}

fn main() {
    let height = vec![0, 2, 0, 3, 1, 0, 1, 3, 2, 1]; 
    println!("{:?}", Solution::trap_rain(&height));
}
