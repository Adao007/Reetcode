struct Solution; 
impl Solution {
    fn warmer_calculation(temp:& Vec<i32>) -> Vec<i32> {
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
    println!("{:?}", Solution::warmer_calculation(& temp));
    println!("{:?}", predict_warmer(temp));
}

fn predict_warmer(temps: Vec<i32>) -> Vec<i32> {
    let mut days = vec![0; temps.len()]; 
    let mut stack: Vec<[i32; 2]> = Vec::new(); 

    for (index, temp) in temps.iter().enumerate() {
        while !stack.is_empty() && *temp > stack[stack.len() - 1][1] {
            if let Some(day) = stack.pop() {
                let index = index as i32 - day[0];
                days[day[0] as usize] = index;  
            }
        }
        
        stack.push([index as i32, *temp]);
    }

    days
}