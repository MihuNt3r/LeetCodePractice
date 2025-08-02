use std::cmp::max;

fn main() {
    let prices = vec![7,1,5,3,6,4];

    let max_profit = Solution::max_profit(prices);

    println!("Max profit: {}", max_profit);
}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;

        for i in 0..prices.len() {
            for j in i + 1..prices.len() {
                let profit = prices[j] - prices[i];

                max_profit = max(profit, max_profit);
            }
        }

        max_profit
    }

    pub fn max_profit_chat_gpt(prices: Vec<i32>) -> i32 {
        let mut left = 0;  // Buy day
        let mut right = 1; // Sell day
        let mut max_profit = 0;

        while right < prices.len() {
            if prices[right] > prices[left] {
                let profit = prices[right] - prices[left];
                max_profit = max_profit.max(profit);
            } else {
                // Found a better day to buy
                left = right;
            }

            right += 1;
        }

        max_profit
    }
}
