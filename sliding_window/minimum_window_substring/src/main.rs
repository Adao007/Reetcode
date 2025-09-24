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
        let mut key_map: HashMap<char, i32> = HashMap::new(); 
        let mut map: HashMap<char, i32> = HashMap::new(); 
        let mut shortest = "".to_string(); 

        for ch in t.iter() {
            if let Some(existing) = key_map.get_mut(ch) {
                *existing += 1; 
            }
            else {
                key_map.insert(*ch, 1); 
            }
        }

        let mut left = 0; 
        let mut right = t.len() - 1; 
        while right < s.len() {
            if key_map.contains_key(&s[right]) {
                if let Some(existing) = map.get_mut(&s[right]) {
                    *existing += 1; 
                }
                else {
                    map.insert(s[right], 1); 
                }
            }
                
            if map == key_map {
                while !key_map.contains_key(&s[left]) {
                    left += 1;
                }
                
                shortest = s[left..=right].iter().collect(); 
                if let Some(existing) = map.get_mut(&s[left]) {
                    if *existing > 1 {
                        *existing -= 1; 
                        left += 1; 
                    }
                    else {
                        map.remove(&s[left]); 
                        left += 1;
                    }
                }
            }

                right += 1;
            }
        shortest
    }     
}


fn main() {
    let s = "OUZODYXAZV".to_string(); // Should return YXAZ
    let t = "XYZ".to_string(); 

    println!("Minimum String in s, containing t: {:?}", Solution::find_substring(s, t));
}
