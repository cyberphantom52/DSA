pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::Ordering;
    nums.sort_unstable();
    let mut result: Vec<Vec<i32>> = Vec::new();
    let len = nums.len();
    for i in 0..len {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let (mut left, mut right) = (i + 1, len - 1);
        while left < right {
            match nums[i]
                .saturating_add(nums[left])
                .saturating_add(nums[right])
                .cmp(&0) {
                    Ordering::Greater => right -= 1,
                    Ordering::Less => left += 1,
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[left], nums[right]]);
                        left  += 1;

                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                    }
                }
        }
    }
    result
}

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let result = three_sum(nums);
    println!("{:?}", result);
}
