struct Solution; 
impl Solution {
    fn check_paren(str: String) -> bool {
        let str: Vec<char> = str.chars().collect(); 
        let mut stack = Vec::new();
        for paren in str.iter() {
            if *paren == '[' || *paren == '(' || *paren == '{' {
                stack.push(*paren); 
            }
            else {
                if let Some(symbol) = stack.pop() {
                    match symbol {
                        '[' => if *paren != ']' {return false}, 
                        '(' => if *paren != ')' {return false}, 
                        '{' => if *paren != '}' {return false}, 
                        _ => (),
                    }
                }
                else { 
                    return false; 
                }
            }
        }

        true
    }
}

fn main() {
    let s1 = "{}".to_string(); 
    let s2 = "([{}])".to_string(); 
    let s3 = "[(])".to_string(); 
    let s4 = "A".to_string(); 

    assert!(Solution::check_paren(s1));
    assert!(Solution::check_paren(s2));
    assert!(!Solution::check_paren(s3));
    assert!(!Solution::check_paren(s4));
    println!("All checks were correct!");
}

