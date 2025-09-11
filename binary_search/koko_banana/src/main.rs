struct Solution; 
impl Solution {
    fn find_good_eating(bananas:&mut Vec<i32>, hours: i32) -> i32 {
        let length = bananas.len() - 1; 
        Self::quick_sort(bananas, 0, length as i32); 
        let right_range = bananas[length]; 
        let range: Vec<i32> = (0..right_range).collect(); 
        Self::binary_search(range, &bananas, hours) 
    }

    fn binary_search(range: Vec<i32>, bananas:& Vec<i32>, hours: i32) -> i32 {
        let mut left: i32 = 1; 
        let mut right:i32 = range.len() as i32;
        let mut mid: i32 = ((right - left) / 2) as i32 + left; 

        while left <= right {
            let mut time_to_eat = 0;
            for i in bananas.iter() {
                let mut time = *i; 
                while time > 0 {
                    time -= mid; 
                    time_to_eat += 1; 
                }
            }

            if time_to_eat == hours {
                return mid;
            }
            else if time_to_eat < hours {
                right = mid - 1;
            }
            else {
                left = mid + 1;
            }

            mid = (right - left) / 2 + left;
        } 

        mid
    }

    fn quick_sort(vector:&mut Vec<i32>, low: i32, high: i32) {
        if low < high {
            let partition = Self::partition(vector, low, high);
            Self::quick_sort(vector, low, partition - 1);
            Self::quick_sort(vector, partition + 1, high); 
        }   
    }

    fn partition(vector:&mut Vec<i32>, low:i32, high:i32) -> i32 {
        // The partition is high! 
        let mut i: i32 = low - 1;
        for j in low..high {
            if vector[j as usize] < vector[high as usize] {
                i += 1; 
                Self::swap(vector, i, j);
            }
        } 
        Self::swap(vector, i + 1, high); 
        i + 1
    }

    fn swap(vector:&mut Vec<i32>, first: i32, second: i32) {
        let temp = vector[first as usize];
        vector[first as usize] = vector[second as usize];
        vector[second as usize] = temp; 
    }
}

fn main() {
    let mut bananas = vec![3, 1]; 
    let hours = 4; 
    println!("The minimum rate of banana eating is {:?}", Solution::find_good_eating(&mut bananas, hours));
}