// On Reddit, a horror story about printing the prime numbers between 0 - 50... 
// Lets implement that! 
fn main() {
    // This is the brute force method...
    // for i in 2..=50 {
    //   let mut count = 2;
    //    while count != 51 {
    //        if count != i {
    //            if i % count == 0 {
    //                break;
    //            } 
    //        }
    //        count += 1; 
    //        if count == 50 {
    //            println!("{}", i); 
    //        }
    //    } 
    //}
    
    is_prime(); 
}

// A prime number is defined by its only possible divisors are 1 and 
// Itself with no remainders. 

fn is_prime() {
    for i in 2..=50 {
        if i == 2 || i == 3 || i == 5 || i == 7 {
            println!{"{}", i}; 
        }       
        if i % 2 != 0 && i % 3 != 0 && i % 5 != 0 && i % 7 != 0 {
            println!("{}", i); 
        }
    }
}