// Generate Parentheses
struct Solution; 
impl Solution {
    fn generate(pairs: i32) -> Vec<String> {
        let mut stack = Vec::new(); 
        let mut res = Vec::new(); 

        Self::backtrack(&mut stack, &mut res, pairs, 0, 0); 
        res
    } 

    fn backtrack(stack: &mut Vec<char>, res: &mut Vec<String>, pairs: i32, front: i32, back: i32) {
        if front == back && front == pairs {
            res.push(stack.iter().collect());
            return ();
        }

        if front < pairs {
            stack.push('('); 
            Self::backtrack(stack, res, pairs, front + 1, back); 
            stack.pop(); 
        }

        if back < front {
            stack.push(')');
            Self::backtrack(stack, res, pairs, front, back + 1); 
            stack.pop(); 
        }
    }
}


fn main() {
    let input = 3;
    println!("{:?}", Solution::generate(input)); 
}
