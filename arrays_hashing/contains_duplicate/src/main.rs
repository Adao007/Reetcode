pub struct Solution; 
impl Solution {
    fn has_duplicate(nums: &Vec<i32>) -> bool {
        let mut checker: Vec<i32> = Vec::new();
        for num in nums.iter() {
            if checker.contains(num) {
                return true;
            }
            else {
                checker.push(*num);
            }
        }
        
        return false
    }
}


fn main() {
    let test_one: Vec<i32> = vec![1, 2, 3, 5]; // Contains No Duplicates
    let test_two: Vec<i32> = vec![1, 2, 3, 3]; // Contains Duplicates

    if Solution::has_duplicate(&test_one) {
        println!("This should not return true, test_one has no duplicates");
    }

    if Solution::has_duplicate(&test_two) {
        println!("Program has caught duplicate array: test_two!");
    }
}
