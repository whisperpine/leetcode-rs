// 121. Best Time to Buy and Sell Stock
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/
// Topics: Sliding Window.
// Difficulty: Easy.

#[test]
fn test_121_best_time_to_buy_and_sell_stock() {}

#[derive(Debug)]
pub struct Solution;

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = prices[0];
        let mut profit = 0;
        for &item in prices.iter().skip(1) {
            if item < buy {
                buy = item;
                continue;
            }
            if item - buy > profit {
                profit = item - buy;
            }
        }
        profit
    }
}
