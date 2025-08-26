// Evaluate Reverse Polish Notation - "erpn"
// In Rust, the stack can be achieved using the vec data structure 
struct Solution; 
impl Solution {
    fn evaluate(tokens: &Vec<char>) -> i32 {
        let mut stack: Vec<i32> = Vec::with_capacity(tokens.len()); 
    
        for token in tokens.iter() {
            if token.is_numeric() {
                if let Some(number) = token.to_digit(10) {
                    stack.push(number as i32); 
                }
            } 
            else {
                match token {
                    '+' => Self::add(&mut stack), 
                    '-' => Self::subtract(&mut stack),
                    '*' => Self::multiply(&mut stack), 
                    '/' => Self::divide(&mut stack),
                    _ => {
                        println!("Evaluate method has found an invalid symbol: {}", token); 
                        return 0; 
                    }
                }
            }
        }

        if let Some(answer) = stack.pop() {
            answer
        }
        else {
            println!("An error has occurred!"); 
            0
        }
    }

    fn add(stack: &mut Vec<i32>) -> () {
        if let Some(right) = stack.pop() {
            if let Some(left) = stack.pop() {
                let ans = left + right; 
                stack.push(ans);
            }
        }
    }

    fn subtract(stack: &mut Vec<i32>) -> () {
        if let Some(right) = stack.pop() {
            if let Some(left) = stack.pop() {
                let ans = left - right; 
                stack.push(ans);
            }
        }
    } 

    fn multiply(stack: &mut Vec<i32>) -> () {
        if let Some(right) = stack.pop() {
            if let Some(left) = stack.pop() {
                let ans = left * right;
                stack.push(ans);
            }
        }
    }

    fn divide(stack: &mut Vec<i32>) -> () {
        if let Some(right) = stack.pop() {
            if let Some(left) = stack.pop() {
                let ans = left / right; 
                stack.push(ans); 
            }
         }
    }
}


fn main() {
    let tokens: Vec<char> = vec!['1', '2', '+', '3', '*', '4', '-'];
    let answer = Solution::evaluate(&tokens); 
    assert_eq!(answer, 2); 
    println!("Evaluation completed! The anwser is {}", answer); 
}
