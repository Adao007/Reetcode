struct Solution; 
impl Solution {
    fn is_valid(s: String) -> bool {
        let s: Vec<char> = s.to_lowercase().chars().filter(|c| !c.is_whitespace()).collect();
        let mut left_ptr = 0; 
        let mut right_ptr = s.len() - 1; 
        while left_ptr < right_ptr {
            if s[left_ptr] != s[right_ptr] {
                return false;
            }

            left_ptr += 1;
            right_ptr -= 1;
        }
        
        true
    }
}

fn main() {
    let s = "Was it a car or a cat I saw".to_string(); 
    assert!(Solution::is_valid(s));
    let s = "tab a cat".to_string(); 
    assert!(!Solution::is_valid(s)); 
    println!("All examples have been accounted for!"); 
}
