struct Solution;
impl Solution {
    fn encode(strs: &Vec<Vec<char>>) -> Vec<char> {  
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

    fn decode(strs: &Vec<char>) -> Vec<String> {
        let mut decode_str: Vec<String> = Vec::new(); 
        let mut i = 0; 
        while i < strs.len() - 1 {
            if strs[i].is_digit(10) && strs[i + 1] == '#' {
                let mut decode = Vec::new(); 
                let mut start = i + 2;
                let mut counter = strs[i].to_digit(10).unwrap();
                
                while counter != 0 {
                    decode.push(strs[start]);
                    start += 1;  
                    counter -= 1; 
                }
                i = start; 
                decode_str.push(decode.into_iter().collect());
            }   
        }

        decode_str
    }
}

fn main() {
    // Example 1:
    let input: Vec<Vec<char>> = vec!["neet".chars().collect(), "code".chars().collect(), "love".chars().collect(), "you".chars().collect()];
    let encoded = Solution::encode(&input); 
    let decoded = Solution::decode(&encoded);
    println!("Encoded String for Example 1:\n {:?}", encoded);
    println!("Decoded Strings for Example 2:\n {:?}", decoded);
    
    // Example 2: 
    let input = vec!["we".chars().collect(), "say".chars().collect(), ":".chars().collect(), "yes".chars().collect()];
    //let encoded = Solution::encode(&input);
    //let decoded = Solution::decode(&encoded);
    let encoded = encode(&input);
    let decoded = decode(&encoded);
    
    println!("Encoded String for Example 2:\n {:?}", encoded);
    println!("Decoded Strings for Example 2:\n {:?}", decoded);
}

fn encode (strs: &Vec<Vec<char>>) -> Vec<char> {
    let mut encoded: Vec<char> = Vec::new();
    for str in strs.iter() {
        encoded.push(char::from_digit(str.len() as u32, 10).unwrap());
        encoded.push('#'); 
        for letters in str.iter() {
            encoded.push(*letters); 
        }
    }

    encoded
}

fn decode (str: &Vec<char>) -> Vec<String> {
    let mut decoded: Vec<String> = Vec::new();
    let mut i = 0; 
    while i < str.len() {
        if str[i].is_digit(10) && str[i + 1] == '#' {
            let count = str[i].to_digit(10).unwrap();
            let mut word = Vec::new(); 
            i += 2;
            for letters in i..(count as usize + i) {
                word.push(str[i]);
                i += 1; 
            }
            decoded.push(word.into_iter().collect());
        }
    }

    decoded
}