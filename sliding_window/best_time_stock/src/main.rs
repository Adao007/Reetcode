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
        let mut right = 1; 
        let mut sell = right; 

        while right < prices.len() {
            if prices[left] > prices[right] {
                left = right; 
            }

            if prices[sell] < prices[right] {
                sell = right;
            }
            right += 1; 
        }

        if sell > left{
            prices[sell] - prices[left]
        }
        else {
            0
        }
    }
}

fn main() {
    let prices = vec![10, 1, 5, 6, 7, 1]; 
    let second_prices = vec![10, 8, 7, 5, 2]; 

    println!("Profit: {}", Solution::best_stock(&prices));
    println!("Profit 2: {}", Solution::best_stock(&second_prices)); 
}
