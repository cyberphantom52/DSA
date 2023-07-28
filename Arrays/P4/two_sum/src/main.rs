pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    for i in 0..nums.len() {
        if !map.contains_key(&(target - nums[i])) {
            map.insert(nums[i], i);
        } else {
            return vec![
                map.get(&(target - nums[i])).unwrap().clone() as i32,
                i as i32,
            ];
        }
    }
    unreachable!();
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums, target);
    println!("{:?}", result);
}
