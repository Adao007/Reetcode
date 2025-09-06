use std::collections::HashMap; 

struct TimeMap{
    map: HashMap<String, Vec<String>>,
} 

impl TimeMap {
    // Store key WITH the value at time: timestamp
    // set should be O(1).
    fn set(&mut self, key: String, value: String, timestamp: i32) -> () {
        let mut vector = Vec::with_capacity(1000); 
        vector[timestamp as usize] = value; 
        self.map.insert(key, vector); 
    }

    // Returns the most recent value of key of the most recent timestamp
    // Else get will return "", get should be O(log n).
    fn get(key: String, timestamp: i32) -> String {
        todo!(); 
    }
}

fn main() {
    let timemap = TimeMap{
        map: HashMap::new(), 
    };
}
