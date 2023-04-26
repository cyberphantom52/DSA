pub fn max_profit(prices: Vec<i32>) -> i32 {
    let size = prices.len();
    let mut min = prices[0];
    let mut max_profit = 0;

    for i in 0..size {
        min = i32::min(min, prices[i]);
        max_profit = i32::max(max_profit, prices[i] - min);
    }

    max_profit
}

fn main() {
    let prices = vec![7,1,5,3,6,4];
    println!("Max Profit : {}", max_profit(prices));
}
