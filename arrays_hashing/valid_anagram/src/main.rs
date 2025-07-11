// Plan: Using a hashtable as a uplook to count each of the chars in the string seems to be a good idea. 
// In Rust, we can use enumerate to vectorize the string.
use std::collections::HashMap;

struct ValidAnagram;
impl ValidAnagram {
    fn check_validity (s: &str, t: &str) -> bool { // Ensure that s is larger than or equal to t
        let mut map: HashMap<char, u32>= HashMap::new();
        let s: Vec<char> = s.chars().collect(); // collects the chars in s and converts it into a vector! 
        let t: Vec<char> = t.chars().collect();

        for character in s.iter() {
            if let Some(value) = map.get_mut(&character) {
                *value += 1;
            }
            else {
                map.insert(*character, 1);
            }
        }

        for character in t.iter() {
            if let Some(value) = map.get_mut(&character) {
                *value -= 1;
                if *value == 0 {
                    map.remove(&character);
                }
            }
            else {
                return false;
            }
        }

        if map.is_empty() { 
            true
        }
        else {
            false
        }
        
    }
}

fn main() {
    let s: String = String::from("racecar");
    let t: String = "carrace".to_string();

    let s2 = "jar";
    let t2 = "jam";

    // Valid Anagram is a function that given two strings will return true if they are anagrams.
    // Anagrams: a string that contains the exact same characters as another string, ordering can be different.
    if ValidAnagram::check_validity(&s, &t) {
        println!("Example 1 is an anagram!"); 
    }
    else {
        println!("Example 1 is not an anagram!");
    }

    if ValidAnagram::check_validity(s2, t2) {
        println!("Example 2 is an anagram!"); 
    }
    else {
        println!("Example 2 is not an anagram!");
    }
} 
