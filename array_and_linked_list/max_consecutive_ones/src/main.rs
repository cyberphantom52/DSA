pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    nums.iter()
        .fold(0, |acc, &x| {
            if x == 1 {
                count += 1;
            } else {
                count = 0;
            }
            acc.max(count)
        })
}

fn main() {
    let nums = vec![1, 1, 0, 1, 1, 1];
    let result = find_max_consecutive_ones(nums);
    println!("result = {}", result);
}
