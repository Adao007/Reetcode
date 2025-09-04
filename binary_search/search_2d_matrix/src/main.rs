struct Solution; 
impl Solution {
    fn search_martix(matrix: &Vec<Vec<i32>>, target: i32) -> bool {
        let row = Self::binary_search(&column, target);
        let mark = Self::binary_search(&matrix[row as usize], target);

        if matrix[row as usize][mark as usize] == target {
            return true; 
        }

        false 
    }

    fn binary_search(nums: &Vec<Vec<i32>>, target: i32) -> i32 {
        let mut left_ptr = 0; 
        let mut right_ptr = nums.len() - 1; 
        let mut mid = ((right_ptr - left_ptr) / 2) + left_ptr; 

        while left_ptr < right_ptr {
            if nums[mid] == target {
                return mid as i32;
            }
            else if nums[mid] < target {
                left_ptr = mid + 1; 
            }
            else {
                right_ptr = mid - 1; 
            }
            
            mid = ((right_ptr - left_ptr) / 2) + left_ptr; 
        }

        mid as i32
    }
}

fn main() {
    let matrix = vec![vec![1, 2, 4, 8], vec![10, 11, 12, 13], vec![14, 20, 30, 40]]; 
    let target = 13; 
    assert!(Solution::search_martix(&matrix, target)); 
    println!("All is working as intended!"); 
}