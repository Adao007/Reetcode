struct Solution; 
impl Solution {
    fn search_matrix(matrix: &[[i32; 4]; 3], target: i32) -> bool {
        // search the column to find the closest row to target 
        let mut left = 0; 
        let mut right = matrix.len(); 
        let mut mid = ((right - left) / 2 ) + left; 
        let mut last_row = 0; 
        while left < right {
            if matrix[mid][0] == target {
                return true;
            }
            else if matrix[mid][0] > target {
                mid = last_row;
                break; 
            }
            else {
                left = mid + 1; 
            }

            last_row = mid; 
            mid = ((right - left) / 2) + left;
        }
        
        // Search the row to find the target 
        let row = mid; 
        left = 0; 
        right = matrix[row].len();
        mid = ((right - left) / 2) + left; 
        println!("{} {} {} {}", row, left, right, mid);
        while left < right {
            if matrix[row][mid] == target {
                return true; 
            }
            else if matrix[row][mid] > target {
                right = mid - 1; 
            }
            else {
                left = mid + 1; 
            }

            mid = ((right - left) / 2) + left; 
        }
        
        false 
    }

    
}

fn main() {
    let matrix:[[i32; 4]; 3] = [[1, 2, 4, 8], [10, 11, 12, 13], [14, 20, 30, 40]]; 
    let target = 30; 
    assert!(Solution::search_matrix(&matrix, target)); 
}
