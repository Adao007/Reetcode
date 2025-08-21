use std::ptr::read_unaligned;

struct Solution; 
impl Solution {
    fn trap_rain(height: Vec<i32>) -> i32 {
        let mut rain = 0; 
        let left_ptr = 1; 
        let right_ptr = height.len() - 1; 

        while left_ptr < right_ptr {
            let wall = height[left_ptr - 1].min(height[right_ptr]);
            let water = wall - height[left_ptr];
            if water > 0 {
                rain += water; 
            }
        }

        rain
    }
}

fn main() {
    let height = vec![0, 2, 0, 3, 1, 0, 1, 3, 2, 1]; 
    println!("{:?}", Solution::trap_rain(&height));
}
