struct Solution;
impl Solution {
    fn find_products(nums: &[i32; 4]) -> Vec<i32> {
        let mut products = vec![];
        let mut fix = 1; 

        for num in nums.iter() {
            products.push(fix);
            fix *= num; 
        }

        fix = 1; 
        for (pos, num) in nums.iter().enumerate().rev() {
            products[pos] *= fix; 
            fix *= num;
        }

        products
    }
}

fn main() {
    let nums = [1, 2, 4, 6];
    println!("{:?}", Solution::find_products(&nums));
}
