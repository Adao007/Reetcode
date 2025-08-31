struct Solution; 
impl Solution {
    fn generate_parentheses(number: i32) -> Vec<String> {
        // Use number as limits for front and back parentheses. 
        let mut res = Vec::new(); 
        let mut stack: Vec<char> = Vec::new();

        fn generate(res:&mut Vec<String>, stack:&mut Vec<char>, number: i32, front: i32, back: i32) {
            if front == back && back == number {
                res.push(stack.iter().collect()); 
            }

            if front < number {
                stack.push('('); 
                generate(res, stack, number, front + 1, back);
                stack.pop(); 
            }

            if back < front {
                stack.push(')');
                generate(res, stack, number, front, back + 1);
                stack.pop();
            }
        }

        generate(&mut res, &mut stack, number, 0, 0); 
        res
    }
}

fn main() {
    let input = 3; 
    println!("{:?}", Solution::generate_parentheses(input)); 
}
