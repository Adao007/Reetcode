struct Solution; 
impl Solution {
    fn find_min(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0; 
        let mut right: i32 = nums.len() as i32 - 1; 
        let mut mid: i32 = ((right - left)/ 2) + left;

        while left <= right {
            if nums[mid as usize] == target {
                return mid as i32; 
            }
            
            if nums[left as usize] <= nums[mid as usize] {
                if nums[left as usize] <= target && target < nums[mid as usize] {
                    right = mid - 1;
                }
                else {
                    left = mid + 1; 
                }
            }
            else {
                if nums[mid as usize] < target && target <= nums[right as usize] {
                    left = mid + 1; 
                }
                else {
                    right = mid - 1; 
                }
            }

            mid = ((right - left) as i32 / 2) + left as i32; 
        } 

        -1
    }
}

fn main() {
    let nums = vec![3, 4, 5, 6, 1, 2]; 
    let target = 1; 

    let nums2 = vec![3, 5, 6, 0, 1, 2]; 
    let target2 = 4; 

    let nums3 = vec![1, 2, 3, 4, 5, 6]; 
    let target3 = 6; 

    let nums4 = vec![5, 1, 2]; 
    let target4 = 5; 

    println!("{:?}", Solution::find_min(&nums, target)); 
    println!("{:?}", Solution::find_min(&nums2, target2));
    println!("{:?}", Solution::find_min(&nums3, target3));
    println!("{:?}", Solution::find_min(&nums4, target4)); 
}
