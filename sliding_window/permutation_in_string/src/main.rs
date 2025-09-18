/*
    You are given two strings s1 and s2. 
    Return true if s2 contains a permutation of s1, or false otherwise. 
    That means if a permutaton of s1 exists as a substring of s2,
    then return true. 
*/

struct Solution; 
impl Solution {
    // Brute Force Method
    fn find_permutation(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false; 
        }        

        let s1: Vec<char> = s1.chars().collect(); 
        let s2: Vec<char> = s2.chars().collect(); 
        let mut frequency: [i32; 26] = [0; 26]; 
        
        for ch in s1.iter() {
            frequency[((*ch as u32) - ('a' as u32)) as usize] += 1; 
        }

        let mut left = 0; 
        let mut right = s1.len() - 1; 

        while right < s2.len() {
            let mut check_freq: [i32; 26] = [0;26];
            for i in left..=right {
                check_freq[((s2[i] as u32) - ('a' as u32)) as usize] += 1; 
            }
            if check_freq == frequency {
                return true; 
            }

            left += 1; 
            right += 1; 
        }

        false
    }
}

fn main() {
    let s1 = "abc".to_string(); 
    let s2 = "lecabee".to_string(); 

    let s3 = String::from("abs"); 
    let s4 = String::from("lecaabee"); 

    println!("Does s2 contain a permutation of s1? {:?}", Solution::find_permutation(s1, s2));
}
