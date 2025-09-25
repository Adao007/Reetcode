use std::collections::BinaryHeap;

struct Solution; 
impl Solution{
    fn get_maximums(list: Vec<i32>, k: i32) -> Vec<i32> {
        let mut left: usize = 0;
        let mut right = k as usize - 1;
        let mut max_heap = BinaryHeap::new(); 
        let mut maximums = Vec::new(); 

        for index in 0..k {
            let tuple = (list[index as usize], index as usize);
            max_heap.push(tuple);
        }

        while right < list.len() {
            if let Some(max) = max_heap.peek() {
                if max.1 >= left {
                    maximums.push(max.0); 
                    left += 1; 
                    right += 1; 
                    if right != list.len() {
                        max_heap.push((list[right], right)); 
                    }
                }
                else {
                    max_heap.pop(); 
                }
            }
        }
        maximums
    }
}

fn main() {
    let tuple1 = (2, 0); 
    let tuple2 = (1, 1); 
    let tuple3 = (5, 2); 
    let tuple4 = (4, 3); 

    let mut max_heap = BinaryHeap::new(); 
    max_heap.push(tuple1); 
    max_heap.push(tuple2);
    max_heap.push(tuple3);
    max_heap.push(tuple4); 

    max_heap.pop();
    println!("{:?}", max_heap.peek()); 

    // START PROBLEM
    let list = vec![8, 1, 1, 0, 3, 2, 5, 6]; // 8, 1, 3, 3, 5, 6
    let k = 3; 
    println!("List of maximums of {} is {:?}", k, Solution::get_maximums(list, k));
}
