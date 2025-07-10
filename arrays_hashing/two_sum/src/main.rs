use std::collections::HashMap;

struct TwoSum; 
impl TwoSum {
    fn solution(nums: Vec<i32>, target: i32) -> Vec<usize> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut results: Vec<usize> = Vec::new();

        for (i, num) in nums.iter().enumerate() {
            if let Some(value) = map.get(&num) {
                results.push(*value);
                results.push(i);
                return results;
            }
            else {
                let difference = target - num; 
                map.insert(difference, i);
            }
        }

        results
    }
}

fn main() {
    let nums = vec![3, 4, 5, 6];
    let nums2 = vec![4, 5, 6];
    let target = 7;
    let target2 = 10;

    println!("The two indices that sum to {} is {:?}", target, TwoSum::solution(nums, target));
    println!("The two indices that sum to {} is {:?}", target2, TwoSum::solution(nums2, target2));
}
