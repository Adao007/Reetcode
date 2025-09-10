use std::i32::{MIN, MAX};
struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() <= nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };

        let total = a.len() + b.len();
        let half = total / 2;
        
        let mut left = 0;
        let mut right = a.len();
        
        while left <= right {
            let i = (left + right) / 2;
            let j = half - i;
            
            let a_left = if i > 0 { a[i - 1] } else { MIN };
            let a_right = if i < a.len() { a[i] } else { MAX };
            let b_left = if j > 0 { b[j - 1] } else { MIN };
            let b_right = if j < b.len() { b[j] } else { MAX };
            
            if a_left <= b_right && b_left <= a_right {
                if total % 2 == 1 {
                    return a_right.min(b_right) as f64;
                } else {
                    return (a_left.max(b_left) + a_right.min(b_right)) as f64 / 2.0;
                }
            } else if a_left > b_right {
                right = i - 1;
            } else {
                left = i + 1;
            }
        }
        
        unreachable!("Should never reach here for sorted arrays")
    }
}

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec!{2, 4}; 

    let nums3 = vec![1, 2]; 
    let nums4 = vec![3]; 

    println!("{:?}", Solution::find_median_sorted_arrays(nums1, nums2));     
    println!("{:?}", Solution::find_median_sorted_arrays(nums3, nums4)); 
}
