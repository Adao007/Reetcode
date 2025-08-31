struct Solution; 
impl Solution {
    fn get_car_fleets(target: i32, position: &Vec<i32>, speed: &Vec<i32>) -> i32 {

    }
}

fn main() {
    let target = 10; 
    let position = vec![1, 4]; 
    let speed = vec![3, 2]; 

    println!("The number of car fleets that arrived is: {:?}", Solution::get_car_fleets(target, &position, &speed))
}
