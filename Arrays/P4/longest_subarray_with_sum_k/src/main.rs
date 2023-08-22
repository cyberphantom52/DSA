pub fn longest_subarray_with_sum_k(nums: &Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut sum_map: HashMap<i32, usize> = HashMap::new();

    let mut sum = 0;
    let mut max_len = 0;
    for (i, num) in nums.iter().enumerate() {
        sum += num;
        if sum_map.contains_key(&(sum - k)) {
            max_len = std::cmp::max(max_len, i - sum_map.get(&(sum - k)).unwrap());
        }
        if !sum_map.contains_key(&sum) {
            sum_map.insert(sum, i);
        }
    }
    max_len as i32
}

fn main() {
    let nums = vec![1, 2, 3, 1, 1, 1, 1];
    let k = 3;
    let result = longest_subarray_with_sum_k(&nums, k);
    println!("result: {}", result);
}
