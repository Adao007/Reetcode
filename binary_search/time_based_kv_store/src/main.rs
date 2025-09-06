use std::collections::HashMap; 

#[derive(Debug)]
struct TimeMap{
    map: HashMap<String, Vec<(i32, String)>>,
} 

impl TimeMap {
    fn new() -> Self {
        TimeMap { map: HashMap::new() }
    }

    // Store key WITH the value at time: timestamp
    // set should be O(1).
    fn set(&mut self, key: String, value: String, timestamp: i32) -> () {
        if self.map.is_empty() {
            let mut vector: Vec<(i32, String)> = Vec::with_capacity(1001); 
            vector.push((timestamp, value)); 
            self.map.insert(key, vector); 
        }
        else {
            if let Some(timeline) = self.map.get_mut(&key) {
                timeline.push((timestamp, value));
            }
        }
    }

    // Returns the most recent value of key of the most recent timestamp
    // Else get will return "", get should be O(log n).
    fn get(&mut self, key: String, timestamp: i32) -> String {
        if let Some(timeline) = self.map.get(&key) {
            let recent = Self::binary_search(timeline, timestamp); 
            return timeline[recent as usize].1.clone(); 
        }
        
        String::from("")
    }

    fn binary_search(timeline: &Vec<(i32, String)>, timestamp: i32) -> i32 {
        let mut left_ptr = 0; 
        let mut right_ptr = timeline.len() - 1; 
        let mut mid_point: i32 = ((right_ptr - left_ptr) as i32 / 2) + left_ptr as i32; 
        let mut difference = 1001; 
        let mut closest_index: usize = 1001; 

        while left_ptr <= right_ptr {
            if timeline[mid_point as usize].0 == timestamp {
                return mid_point as i32; 
            } 
            else if timeline[mid_point as usize].0 > timestamp {
                let check = timeline[mid_point as usize].0 - timestamp; 
                if difference > check {
                    difference = check;
                    closest_index = mid_point as usize;  
                }
                right_ptr = mid_point as usize - 1;
            }
            else {
                let check = timestamp - timeline[mid_point as usize].0;
                if difference > check {
                    difference = check; 
                    closest_index = mid_point as usize; 
                } 
                left_ptr = mid_point as usize + 1; 
            }

            mid_point = ((right_ptr as i32 - left_ptr as i32) / 2) + left_ptr as i32;
        }
        
        closest_index as i32
    }
}

fn main() {
    let mut timemap = TimeMap::new(); 

    let name = String::from("jupi"); 
    let value = String::from("happy");
    let timestamp = 1; 
    timemap.set(name, value, timestamp);
    println!("{:?}", timemap.get(String::from("jupi"), 1));
    println!("{:?}", timemap.get(String::from("jupi"), 2));
    
    let name = String::from("jupi");
    let value = String::from("sad"); 
    let timestamp = 3; 
    timemap.set(name, value, timestamp); 
    println!("{:?}", timemap.get(String::from("jupi"), 3));
}