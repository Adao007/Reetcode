use std::collections::HashMap;

struct GroupAnagrams; 
impl GroupAnagrams {
    fn group_words (strs:& Vec<String>) -> Vec<Vec<String>> 
    {
        // use a container with 26 length as the key. 
        // store strings in new section or in made lookup
        let mut map: HashMap<[i32; 26], Vec<String>> = HashMap::new(); 

        for word in strs {
            let mut array = [0; 26];
            let letters: Vec<char> = word.chars().collect(); 
            for letter in letters.iter() {
                let alpha_pos = (*letter as u32) - ('a' as u32);
                array[alpha_pos as usize] += 1;
            }
            if let Some(key) = map.get_mut(&array) {
                key.push(word.to_string());
            }
            else {
                map.insert(array, vec!(word.to_string()));
            }
        } 

        map.into_values().collect()
    }
}

fn main() {
    let str1 = vec!("act".to_string(), "pots".to_string(), "tops".to_string(), "cat".to_string(), "stop".to_string(), "hat".to_string());
    let str2 = vec!("x".to_string());
    let str3 = vec!("".to_string()); 

    println!("{:?}", group_anagrams(&str1));
    println!("{:?}", GroupAnagrams::group_words(&str1));
    println!("{:?}", GroupAnagrams::group_words(&str2));
    println!("{:?}", GroupAnagrams::group_words(&str3));
}

fn group_anagrams(strs: &Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<[i32; 26], Vec<String>> = HashMap::new(); 
    for str in strs.iter() {
        let letters: Vec<char> = str.chars().collect(); 
        let mut k = [0; 26];
        for letter in letters.iter() {
            k[((*letter as u32) - ('a' as u32)) as usize] += 1; 
        }

        if let Some(group) = map.get_mut(&k) {
            group.push(str.to_string());
        } 
        else {                
            map.insert(k, vec!(str.to_string()));
        }
    }

    map.into_values().collect()
}