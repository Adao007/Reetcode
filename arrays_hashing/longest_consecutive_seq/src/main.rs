use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution; 
impl Solution {
    fn get_longest(nums: Vec<i32>) -> i32 {
        let set = Self::vec_to_set(nums);
        let mut longest = 0;

        for num in set.iter() {
            if !set.contains(&(*num - 1)) {
                let mut length = 0;
                while set.contains(&(num + length)) {
                    length += 1; 
                }

                longest = longest.max(length); 
            } 
        }

        longest
    }

    fn vec_to_set(vec: Vec<i32>) -> HashSet<i32> {
        HashSet::from_iter(vec)
    }
}

fn main() {
    let nums = vec![2, 20, 4, 10, 3, 4, 5];
    println!("The longest consecutive sequence is {:?}", Solution::get_longest(nums));
}
 