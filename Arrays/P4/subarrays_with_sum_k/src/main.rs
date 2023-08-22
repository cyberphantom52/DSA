pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    let (mut sum, mut count) = (0, 0);
    let mut sum_map = HashMap::from([(0, 1)]);
    
    for num in nums {
        sum += num;
        if let Some(&occurance) = sum_map.get(&(sum - k)) {
            count += occurance;
        }
        *sum_map.entry(sum).or_insert(0) += 1;
    }
    count as i32
}

fn main() {
    // let nums = vec![1,1,1];
    let nums = vec![1,2,3];
    let k = 3;
    let result = subarray_sum(nums, k);
    println!("result: {}", result);
}
