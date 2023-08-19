pub fn subarrays_with_k_xor(nums: Vec<i32>, xor: i32) -> i32 {
    use std::collections::HashMap;
    let (mut xr, mut count) = (0 ,0);
    let mut xor_map = HashMap::from([(0, 1)]);
    for num in nums.iter() {
        xr ^= num;

        if let Some(&occurance) = xor_map.get(&(xr ^ xor)) {
            count += occurance;
        }

        *xor_map.entry(xr).or_insert(0) += 1;
    }
    count
}

fn main() {
    let nums = vec![4, 2, 2, 6, 4];
    let xor = 6;
    let result = subarrays_with_k_xor(nums, xor);
    println!("result = {}", result);
}
