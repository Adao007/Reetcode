use std::collections::HashSet; 

struct Solution; 
impl Solution {
    fn longest_substring(s: &String) -> i32 {
        let string: Vec<char> = s.chars().collect(); 
        let mut longest = 0; 
        let mut left = 0; 
        let mut set: HashSet<char> = HashSet::new(); 

        for right in 0..string.len() {
            if set.contains(&string[right]) {
                set.remove(&string[left]);
                left += 1; 
            }

            longest = longest.max(right - left + 1); 
            set.insert(string[right]); 
        }

        longest as i32 
    }
}

fn main() {
    let s = "zxyzxyz".to_string(); 
    let s2 = String::from("xxxx"); 
    println!("{}", Solution::longest_substring(&s)); println!("Review: {}", longest_substring(s));
    println!("{}", Solution::longest_substring(&s2)); println!("Review: {}", longest_substring(s2));
}

fn longest_substring(s: String) -> i32 {
    let string: Vec<char> = s.chars().collect(); 
    let mut map: HashSet<char> = HashSet::new(); 
    let mut left = 0; 
    let mut longest = 0; 

    for right in 0..string.len() {
        while map.contains(&string[right]) {
            map.remove(&string[left]); 
            left += 1; 
        }

        map.insert(string[right]); 
        longest = longest.max(right - left + 1); 
    }

    longest as i32
}