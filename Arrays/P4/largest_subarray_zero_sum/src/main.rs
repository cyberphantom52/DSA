use std::collections::HashMap;
pub fn max_len(nums: &Vec<i32>) -> i32 {
    let mut sum_map: HashMap<i32, usize> = HashMap::new();

    let mut sum = 0;
    let mut max_len = 0;
    for (i, num) in nums.iter().enumerate() {
        sum += num;
        max_len = std::cmp::max(max_len, i - *sum_map.entry(sum).or_insert(i));
    }
    max_len as i32
}

fn main() {
    let nums = vec![15, -2, 2, -8, 1, 7, 10, 23];
    println!("{}", max_len(&nums));
}
