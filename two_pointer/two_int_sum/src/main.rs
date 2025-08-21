struct Solution; 
impl Solution {
    fn get_two_sum(nums: &Vec<i32>, target: &i32) -> Vec<i32> {
        let mut left_ptr = 0;
        let mut right_ptr = nums.len() - 1; 
        let mut solution = Vec::new(); 

        while left_ptr < right_ptr {
            let sum = nums[left_ptr] + nums[right_ptr];
            
            if sum == *target {
                solution.push((left_ptr + 1) as i32);
                solution.push((right_ptr + 1) as i32); 
                return solution; 
            }
            else if sum > *target {
                right_ptr -= 1;
            }
            else {
                left_ptr += 1; 
            }
        }
        
        solution
    }
}

fn main() {
    let mut numbers = vec![1, 2, 3, 4];
    let mut target = 3; 

    println!("{:?}", Solution::get_two_sum(&numbers, &target));

    numbers = vec![5, 5, 6, 13]; 
    target = 19; 
    println!("{:?}", Solution::get_two_sum(&numbers, &target));
}
