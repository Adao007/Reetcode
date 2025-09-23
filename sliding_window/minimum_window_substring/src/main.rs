use std::collections::HashMap; 

// Given two strings s and t, return the shortest substring of s such that every character in t,
// including duplicates, is present in the substring. If such a substring does not exist, 
// return an empty string "". You may assume that the correct output is always unique. 

struct Solution;
impl Solution {
    fn find_substring(s: String, t: String) -> String {
        if t.len() > s.len() {
            return String::from(""); 
        }

        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect(); 
        
        
        String::from("")
    }
}

fn main() {
    let s = "OUZODYXAZV".to_string(); // Should return YXAZ
    let t = "XYZ".to_string(); 
}
