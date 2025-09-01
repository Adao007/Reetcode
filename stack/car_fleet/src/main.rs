struct Solution; 
impl Solution {
    fn get_car_fleets(target: i32, position: &Vec<i32>, speed: &Vec<i32>) -> i32 {
        let mut fleets: Vec<[&i32; 2]> = Vec::new(); 
        let mut cars = Vec::new();

        // Zip position and speed into cars 
        for (p, s) in position.iter().zip(speed.iter()) {
            cars.push([p, s]); 
        }
        cars.sort(); 
        println!("{:?}", cars); 

        for car in cars.into_iter().rev() {
            fleets.push(car);
            
            if fleets.len() >= 2 {
                if ((target - fleets[fleets.len() - 2][0]) / fleets[fleets.len() - 2][1]) >= 
                ((target - fleets[fleets.len() - 1][0]) / fleets[fleets.len() - 1][1]) {
                    fleets.pop(); 
                }
            }  
        }

        fleets.len() as i32
    }
}

fn main() {
    let target = 10; 
    let position = vec![1, 4]; 
    let speed = vec![3, 2]; 

    // Example 2
    let position2 = vec![4, 1, 0, 7];
    let speed2 = vec![2, 2, 1, 1]; 

    println!("The number of car fleets that arrived is: {:?}", Solution::get_car_fleets(target, &position, &speed)); 
    println!("The number of care fleets in the second example is {:?}", Solution::get_car_fleets(target, &position2, &speed2)); 

    // Review Test
    let t = 100; 
    let p = vec![25, 50, 75]; 
    let s = vec![75, 50, 20]; 

    println!("The number of car fleets that arrived is: {:?}", get_car_fleets(t, &p, &s));
}

fn get_car_fleets(target: i32, position: &Vec<i32>, speed: &Vec<i32>) -> i32 {
    let mut fleets: Vec<[i32; 2]> = Vec::new();
    let mut cars = Vec::new(); 

    for (p, s) in position.iter().zip(speed.iter()) {
        cars.push([*p, *s]); 
    }
    cars.sort(); 

    for car in cars.iter().rev() {
        fleets.push(*car); 
        if fleets.len() >= 2 {
            // car tuple: (Position = 0, Speed = 1)
            let lead = fleets[fleets.len() - 2];
            let follow = fleets[fleets.len() - 1]; 

            // Calculate Time to get to target -> Target - Position / Speed
            if (target - follow[0]) / follow[1] <= (target - lead[0]) / lead[1] {
                fleets.pop(); 
            } 
        }
    }

    fleets.len() as i32
}