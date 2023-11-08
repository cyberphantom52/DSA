pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }
    i32::max(helper(&nums[1..]), helper(&nums[..(n - 1)]))
}

pub fn helper(nums: &[i32]) -> i32 {
    let mut cache = nums.to_vec();
    for (i, &val) in nums.iter().enumerate().rev().skip(1) {
        for j in (i + 2)..nums.len() {
            cache[i] = cache[i].max(val + cache[j]);
        }
        cache[i] = i32::max(cache[i], cache[i + 1])
    }
    println!("{:?}", cache);
    cache[0]
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    println!("result = {}", rob(nums));
}
