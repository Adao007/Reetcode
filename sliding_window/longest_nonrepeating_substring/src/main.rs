use std::collections::HashSet; 

struct Solution; 
impl Solution {
    fn longest_substring(s: String) -> i32 {
        let string: Vec<char> = s.chars().collect(); 
        let mut map: HashSet<char> = HashSet::new(); 
        let mut left = 0; 
        let mut right = 0; 
        let mut longest = 0; 

        while right < string.len() {
            while map.contains(&string[right]) {
                map.remove(&string[left]); 
                left += 1; 
                
            }
            map.insert(string[right]); 
            longest = longest.max(right - left + 1); 
            right += 1; 
        }

        longest as i32
    }
}

fn main() {
    let s = "zxyzxyz".to_string(); 
    let s2 = String::from("xxxx"); 
    println!("{}", Solution::longest_substring(s));
    println!("{}", Solution::longest_substring(s2));
}
