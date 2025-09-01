struct Solution; 
impl Solution {
    fn get_largest_rec(heights: &Vec<i32>) -> i32 {
        let mut largest_rec = 0; 
        let mut valid_stack: Vec<[i32; 2]> = Vec::new(); 

        for (index, height) in heights.iter().enumerate() {
            let mut valid_index = index as i32; 
            while !valid_stack.is_empty() && *height < valid_stack[valid_stack.len() - 1][0] {
                if let Some(valid_height) = valid_stack.pop() {
                    valid_index -= 1;
                    let calculation = valid_height[0] * (index as i32 - valid_height[1]); 
                    largest_rec = largest_rec.max(calculation);                   
                }
            }
            
            valid_stack.push([*height, valid_index]); 
        }

        while !valid_stack.is_empty() {
            if let Some(valid_height) = valid_stack.pop() {
                let calculation: i32 = valid_height[0] * (heights.len() as i32 - valid_height[1]);
                largest_rec = largest_rec.max(calculation);
            }
        }

        largest_rec
    }
}

fn main() {
    let mut heights = vec![7, 1, 7, 2, 2, 4]; 
    println!("The largest rectangle for example 1 is {:?}", Solution::get_largest_rec(&heights));
    heights = vec![1, 3, 7]; 
    println!("The largest rectangle for example 2 is {:?}", Solution::get_largest_rec(&heights));
}
