struct Solution;
impl Solution {
    fn encode(strs:& Vec<Vec<char>>) -> Vec<char> {
        let mut encoded_str: Vec<char> = Vec::new(); 
        for str in strs.iter() {
            encoded_str.push(char::from_digit(str.len() as u32, 10).unwrap());
            encoded_str.push('#');
            for letters in str.iter() {
                encoded_str.push(*letters);
            }
        }

        encoded_str
    }

    fn decode(strs: Vec<char>) -> Vec<Vec<char>> {
        todo!(); 

    }
}

fn main() {
    let input: Vec<Vec<char>> = vec!["neet".chars().collect(), "code".chars().collect(), "love".chars().collect(), "you".chars().collect()];
    
    println!("{:?}", Solution::encode(&input));
}