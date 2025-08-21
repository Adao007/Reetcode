struct Solution; 
impl Solution {
    fn three_sums(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut right_ptr = nums.len() - 1; 
        let mut left_ptr = 1; 
        let mut solution: Vec<Vec<i32>> = vec![]; 
        
        for i in 0..=nums.len() - 3 {
            while left_ptr < right_ptr {
                let sum = nums[i] + nums[left_ptr] + nums[right_ptr];
                if sum == 0 {
                    solution.push(vec![nums[i], nums[left_ptr], nums[right_ptr]]); 
                    break;
                }
                else if sum > 0 {
                    right_ptr -= 1;
                }
                else {
                    left_ptr += 1;
                }
            }
            left_ptr = (i + 2) as usize;
            right_ptr = nums.len() - 1;  
        }
        
        solution
    }
}

fn main() {
    let mut numbers = vec![-1, 0, 1, 2, -1, 4]; 
    println!("{:?}", Solution::three_sums(&mut numbers));

    let mut example2 = vec![0, 1, 1]; 
    println!("{:?}", Solution::three_sums(&mut example2));

    let mut example3 = vec![0, 0, 0];
    println!("{:?}", Solution::three_sums(&mut example3));
}
