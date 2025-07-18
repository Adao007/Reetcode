 // Given an array, return the top k frequent elements 
use std::collections::HashMap;

struct Solution; 
impl Solution {
    fn get_top_k(nums:& Vec<i32>, k: usize) -> Vec<i32> {
        // Key: Number, Value: Count 
        let mut frequency_map: HashMap<i32, i32> = HashMap::new(); 
        // The index should equal the Value:Count, place the Key into the Vec at index
        let mut bucket: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1]; 
        let mut k_values: Vec<i32> = vec![];

        for num in nums.iter() {
             frequency_map.entry(*num).and_modify(|counter| *counter += 1).or_insert(1);
        }

        for (key, value) in frequency_map.iter() {
            if *key != 0 {
                bucket[*value as usize].push(*key); 
            }
        }
    
        for nums in bucket.iter_mut().rev() {
            for num in nums.iter_mut() {
                if k_values.len() == k {
                    return k_values;
                }
                
                k_values.push(*num);
            }
        }
        
        k_values
    }
}

fn main() {
    let mut k = 2;
    let mut nums = vec![1, 2, 2, 2, 3, 3, 3];
    println!("{:?}", Solution::get_top_k(&nums, k));

    k = 1; 
    nums = vec![7,7];
    println!("{:?}", Solution::get_top_k(&nums, k))
}
