/*
*   You are given an integer array "prices" where "price[i]" is the price of NeetCoin on the "ith" day
*   you may choose a "single day" to buy one NeetCoin and choose a "different day in the future" to sell it
*   Return the maximum profit you can achieve. You may choose to not make any transactions, 
*   in which case the profit would be 0.
*/

struct Solution; 
impl Solution {
    fn best_stock(prices: &Vec<i32>) -> i32 {
        let mut left = 0; 
        let mut best_price = 0; 

        for right in 0..prices.len() {
            if prices[left] < prices[right] {
                best_price = best_price.max(prices[right] - prices[left]); 
            }

            if prices[left] > prices[right] {
                left = right; 
            }
        }

        best_price
    }
}

fn main() {
    let prices = vec![10, 1, 5, 6, 7, 1]; 
    let second_prices = vec![10, 8, 7, 5, 2]; 

    println!("Profit: {}", Solution::best_stock(&prices)); println!("Review Profit: {}", best_stock(&prices));
    println!("Profit 2: {}", Solution::best_stock(&second_prices)); println!("Review Profit 2: {}", best_stock(&second_prices));
}

// Review Function: 
fn best_stock(prices: &Vec<i32>) -> i32 {
    let mut left = 0;
    let mut best_price = 0; 

    for right in 1..prices.len() {
        if prices[right] > prices[left] {
            best_price = best_price.max(prices[right] - prices[left]); 
        }

        if prices[right] < prices[left] {
            left = right; 
        }
    }
    best_price
}