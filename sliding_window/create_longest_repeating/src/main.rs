/*
    You are given a string s consisting of only uppercase english characters and an integer k. 
    You can choose up to k characters of the string and replace them with any other uppercase English character.

    After performing at most k replacements, 
    return the length of the longest substring which contains only one distinct character.
*/
use std::collections::HashMap; 

struct Solution; 
impl Solution {
    fn get_longest(s: String, k: i32) -> i32 {
        let string: Vec<char> = s.chars().collect(); 
        let mut map: HashMap<char, i32> = HashMap::new(); // Value contains the max frequency... 
        let mut longest = 0; 
        let mut left = 0; 

        for right in 0..string.len() {
            if let Some(existing) = map.get_mut(&string[right]) {
                *existing += 1; 
            } 
            else {
                map.insert(string[right], 1); 
            }

            // Get highest frequent character (of 26 possible characters)
            let mut max_frequency = 0; 
            for value in map.values() {
                max_frequency = max_frequency.max(*value)
            }

            if (right - left + 1) as i32 - max_frequency > k {
                if let Some(existing) = map.get_mut(&string[left]) {
                    *existing -= 1; 
                    left += 1; 
                }
            } 

            longest = longest.max(right - left + 1); 
        }

        longest as i32
    }
}

fn main() {
    let s = String::from("XYYX"); let k = 2; 
    // Output should be 4, YYYY or XXXX after replacing Ys or Xs.
    let s2 = String::from("AAABAB"); let k2 = 1; 
    // Output should be 5, AAAAA after replacing B with A. 

    println!("The longest repeating string has a length of {}", Solution::get_longest(s, k));
    println!("The longest repeating string has a length of {}", Solution::get_longest(s2, k2));
}

