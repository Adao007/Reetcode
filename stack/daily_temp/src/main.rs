struct Solution; 
impl Solution {
    fn warmer_calculation(temp: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<[i32; 2]> = Vec::new(); 
        let mut res = vec![0; temp.len()];

        for (i, t) in temp.iter().enumerate() {
            while !stack.is_empty() && *t > stack[stack.len() - 1][1] {
                if let Some(pair) = stack.pop() {
                    let days = i as i32 - pair[0];
                    res[pair[0] as usize] = days; 
                }
            }

            stack.push([i as i32, *t]);              
        }
    
        res
    }   
}

fn main() {
    let temp = vec![30, 39, 30, 36, 34, 40, 293]; 
    println!("{:?}", Solution::warmer_calculation(temp));
}
