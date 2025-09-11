// Given unsorted arrays, find the median of the two arrays. 
use std::i32::{MIN, MAX};
struct Solution;
impl Solution {
    fn get_median(vec1: &mut Vec<i32>, vec2: &mut Vec<i32>) -> f32 {
        let (a, b) = if vec1.len() <= vec2.len() { (vec1, vec2) } else { (vec2, vec1) }; 
        let total = a.len() + b.len(); 
        let half = total / 2; 
        let mut left = 0; 
        let mut right = a.len() - 1; 
        
        while left <= right {
            let i = (left + right) / 2; 
            let j = half - i; 

            let a_left = if i > 0 { a[i - 1] } else { MIN }; 
            let a_right = if i < a.len() { a[i] } else { MAX }; 
            let b_left  = if j > 0 { b[j - 1] } else { MIN }; 
            let b_right = if j < b.len() { b[j] } else { MAX }; 

            if a_left <= b_right && b_left <= a_right {
                if total % 2 == 0 {
                    return (a_left.max(b_left) + a_right.min(b_right)) as f32 / 2.0; 
                }
                else {
                    return a_right.min(b_right) as f32; 
                }
            }
            else if a_left > b_right {
                right = i - 1;
            }
            else {
                left = i + 1; 
            }
        }
        
        0.0 
    }

    fn quick_sort(vector: &mut Vec<i32>, left: i32, right: i32) {
        if left < right {
            let partition = Self::partition(vector, left, right); 
            Self::quick_sort(vector, left, partition - 1); 
            Self::quick_sort(vector, partition + 1, right); 
        }
    } 

    fn partition(vector:&mut Vec<i32>, low:i32, high:i32) -> i32 {
        // The partition is high! 
        let mut i: i32 = low - 1;
        for j in low..high {
            if vector[j as usize] < vector[high as usize] {
                i += 1; 
                Self::swap(vector, i, j);
            }                                                                                                                                 
        } 
        Self::swap(vector, i + 1, high); 
        i + 1
    }

    fn swap(vector:&mut Vec<i32>, first: i32, second: i32) {
        let temp = vector[first as usize];
        vector[first as usize] = vector[second as usize];
        vector[second as usize] = temp; 
    }

}

fn main() {
    let mut nums1 = vec![3, 1]; // unsorted
    let len = nums1.len() - 1; 
    Solution::quick_sort(&mut nums1, 0, len as i32);
    println!("{:?}", nums1); 
    let mut nums2 = vec!{2, 4}; 
    println!("{:?}", Solution::get_median(&mut nums1, &mut nums2)); 


    let mut nums3 = vec![2, 1]; // unsorted 
    let mut nums4 = vec![3]; 
    let len = nums3.len() - 1; 
    Solution::quick_sort(&mut nums3, 0, len as i32);
    println!("{:?}", nums3); 
    println!("{:?}", Solution::get_median(&mut nums3, &mut nums4)); 
}